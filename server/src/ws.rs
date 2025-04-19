use axum::{
    extract::{Query, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async_tls_with_config, Connector};
use tokio_tungstenite::tungstenite as tt;

#[derive(Debug, serde::Deserialize)]
pub struct ExecParams {
    pub namespace: String,
    pub pod: String,
    pub container: String,
}

pub async fn exec_ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<ExecParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_exec_socket(socket, params))
}

async fn handle_exec_socket(mut client_ws: WebSocket, params: ExecParams) {
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let token = api::utils::get_api_token();

    let k8s_url = format!("wss://{server_host}:6443/api/v1/namespaces/{}/pods/{}/exec?container={}&stdin=1&stdout=1&stderr=1&tty=1&command=sh",
        params.namespace, params.pod, params.container
    );

    let url = match url::Url::parse(&k8s_url) {
        Ok(url) => url,
        Err(e) => {
            let _ = client_ws.send(Message::Text(format!("Invalid K8s URL: {}", e))).await;
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
        .header("Authorization", format!("Bearer {}", token))
        .version(tt::http::Version::HTTP_11)
        .body(()) {
            Ok(req) => req,
            Err(e) => {
            let _ = client_ws.send(Message::Text(format!("Failed to build request: {}", e))).await;
            return;
        }
    };

    let tls = match native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(tls) => tls,
        Err(e) => {
            let _ = client_ws.send(Message::Text(format!("Failed to configure TLS: {}", e))).await;
            return;
        }
    };

    let (k8s_ws_stream, _) = match connect_async_tls_with_config(request, None, false, Some(Connector::NativeTls(tls))).await {
        Ok(res) => res,
        Err(e) => {
            let _ = client_ws.send(Message::Text(format!("Failed to connect to K8s: {e}"))).await;
            return;
        }
    };

    let (mut k8s_sink, mut k8s_stream) = k8s_ws_stream.split();
    let (mut client_sink, mut client_stream) = client_ws.split();

    let to_k8s = async {
        while let Some(Ok(msg)) = client_stream.next().await {
            if let Message::Text(mut t) = msg {
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
                let payload = b.get(1..)
                    .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
                    .unwrap_or_default();
                if !payload.is_empty() {
                    let _ = client_sink.send(Message::Text(payload)).await;
                }
            }
        }
    };

    tokio::join!(to_k8s, to_browser);
}
