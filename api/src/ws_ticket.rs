//! Single-use tickets for `/ws/exec`.
//!
//! A browser's `WebSocket` constructor cannot attach an `Authorization`
//! header or read a session cookie the way a normal request would in every
//! browser (and this deployment doesn't rely on it doing so) - so the actix
//! `Auth` middleware that guards every other route can't guard the WS upgrade
//! directly. Instead: an authenticated caller mints a short-lived ticket
//! through this ordinary (middleware-protected) server function, the client
//! passes only the ticket on the WS URL, and `server/src/ws.rs` redeems it
//! (single use, via `CacheStore::take`) before opening the upstream K8s
//! connection. The ticket - not anything the client sends afterward - is the
//! only thing the WS handler trusts for which pod it's allowed to reach.

use leptos::prelude::ServerFnError;
use leptos::server;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecTicket {
    pub namespace: String,
    pub pod: String,
    pub container: String,
}

/// How long a minted ticket stays redeemable. Long enough to cover the
/// network round trip from minting to the WS upgrade arriving; short enough
/// that a leaked ticket (a browser history entry, a proxy log) is useless
/// well before anyone could act on it.
const TICKET_TTL_SECS: u64 = 30;

#[server(MintExecTicket, "/api/ws/exec-ticket")]
pub async fn mint_exec_ticket(
    namespace: String,
    pod: String,
    container: String,
) -> Result<String, ServerFnError> {
    native::mint(namespace, pod, container).await
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{ExecTicket, TICKET_TTL_SECS};
    use crate::auth::require_write;
    use actix_web::web::Data;
    use leptos::prelude::ServerFnError;
    use quench_cache::CacheStore;
    use uuid::Uuid;

    pub async fn mint(
        namespace: String,
        pod: String,
        container: String,
    ) -> Result<String, ServerFnError> {
        require_write().await?;

        let store = leptos_actix::extract::<Data<CacheStore>>().await?;
        let ticket = Uuid::new_v4().to_string();
        let value = serde_json::to_value(ExecTicket {
            namespace,
            pod,
            container,
        })
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        store
            .set(&ticket_key(&ticket), value, Some(TICKET_TTL_SECS))
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(ticket)
    }

    /// Redeems (and consumes) a ticket minted by [`mint`]. Not a `#[server]`
    /// function - called directly from `server/src/ws.rs`'s plain actix
    /// handler, which has its own `CacheStore` handle.
    pub async fn redeem(store: &CacheStore, ticket: &str) -> Option<ExecTicket> {
        let value = store.take(&ticket_key(ticket)).await.ok().flatten()?;
        serde_json::from_value(value).ok()
    }

    fn ticket_key(ticket: &str) -> String {
        format!("exec-ticket:{ticket}")
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::redeem;
