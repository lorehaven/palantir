use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures::{SinkExt, StreamExt};
use quench_cache::CacheStore;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite as tt, Connector};

#[derive(Debug, serde::Deserialize)]
pub struct TicketParam {
    pub ticket: String,
}

/// Redeems the ticket `api::ws_ticket::mint_exec_ticket` minted for an
/// authenticated caller - see that module for why the WS upgrade can't carry
/// the caller's identity directly. A missing/expired/already-used ticket
/// closes the socket immediately rather than falling back to any
/// client-supplied namespace/pod/container.
pub async fn exec_ws_handler(
    req: HttpRequest,
    body: web::Payload,
    query: web::Query<TicketParam>,
    cache: web::Data<CacheStore>,
) -> Result<HttpResponse, Error> {
    let Some(exec_ticket) = api::ws_ticket::redeem(&cache, &query.ticket).await else {
        return Ok(HttpResponse::Unauthorized().body("invalid or expired exec ticket"));
    };

    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;
    actix_web::rt::spawn(handle_exec_socket(
        session,
        msg_stream,
        exec_ticket,
        cache.into_inner(),
    ));
    Ok(response)
}

async fn handle_exec_socket(
    mut client_session: actix_ws::Session,
    mut client_stream: actix_ws::MessageStream,
    params: api::ws_ticket::ExecTicket,
    cache: std::sync::Arc<CacheStore>,
) {
    let server_host = api::config::server_host();
    let server_port = api::config::server_port();
    let token = api::utils::get_api_token(&cache).await;

    let k8s_url = format!("wss://{server_host}:{server_port}/api/v1/namespaces/{}/pods/{}/exec?container={}&stdin=1&stdout=1&stderr=1&tty=1&command=sh",
        params.namespace, params.pod, params.container
    );

    let url = match url::Url::parse(&k8s_url) {
        Ok(url) => url,
        Err(e) => {
            let _ = client_session.text(format!("Invalid K8s URL: {e}")).await;
            return;
        }
    };

    let request = match tt::http::Request::builder()
        .method("GET")
        .uri(url.as_str())
        .header("Host", url.host_str().unwrap())
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", tt::handshake::client::generate_key())
        .header("Sec-WebSocket-Protocol", "v4.channel.k8s.io")
        .header("Authorization", format!("Bearer {token}"))
        .version(tt::http::Version::HTTP_11)
        .body(())
    {
        Ok(req) => req,
        Err(e) => {
            let _ = client_session
                .text(format!("Failed to build request: {e}"))
                .await;
            return;
        }
    };

    let tls = match native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(tls) => tls,
        Err(e) => {
            let _ = client_session
                .text(format!("Failed to configure TLS: {e}"))
                .await;
            return;
        }
    };

    let (k8s_ws_stream, _) =
        match connect_async_tls_with_config(request, None, false, Some(Connector::NativeTls(tls)))
            .await
        {
            Ok(res) => res,
            Err(e) => {
                let _ = client_session
                    .text(format!("Failed to connect to K8s: {e}"))
                    .await;
                return;
            }
        };

    let (mut k8s_sink, mut k8s_stream) = k8s_ws_stream.split();

    let to_k8s = async {
        while let Some(Ok(msg)) = client_stream.next().await {
            if let Message::Text(t) = msg {
                let mut t = t.to_string();
                if !t.ends_with('\n') {
                    t.push('\n');
                }
                let mut stdin_data = vec![0x00];
                stdin_data.extend(t.as_bytes());
                let _ = k8s_sink
                    .send(tt::Message::Binary(bytes::Bytes::from(stdin_data)))
                    .await;
            }
        }
    };

    let to_browser = async {
        while let Some(Ok(msg)) = k8s_stream.next().await {
            if let tt::Message::Binary(b) = msg {
                let payload = b
                    .get(1..)
                    .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
                    .unwrap_or_default();
                if !payload.is_empty() && client_session.text(payload).await.is_err() {
                    return;
                }
            }
        }
    };

    tokio::join!(to_k8s, to_browser);
}
