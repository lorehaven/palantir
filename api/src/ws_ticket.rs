//! Single-use tickets for `/ws/exec`.
//!
//! A browser's `WebSocket` constructor cannot attach an `Authorization`
//! header or read a session cookie the way a normal request would in every
//! browser (and this deployment doesn't rely on it doing so) - so the actix
//! `Auth` middleware that guards every other route can't guard the WS upgrade
//! directly. Instead: an authenticated caller mints a short-lived ticket
//! through this handler, the client passes only the ticket on the WS URL,
//! and `server/src/ws.rs` redeems it (single use, via `CacheStore::take`)
//! before opening the upstream K8s connection. The ticket - not anything the
//! client sends afterward - is the only thing the WS handler trusts for
//! which pod it's allowed to reach.

use quench_auth::prelude::{Claims, JwtConfig};
use quench_cache::CacheStore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub async fn mint(
    cache: &CacheStore,
    config: &JwtConfig,
    claims: Option<&Claims>,
    namespace: String,
    pod: String,
    container: String,
) -> anyhow::Result<String> {
    crate::auth::require_write(config, claims)?;

    let ticket = Uuid::new_v4().to_string();
    let value = serde_json::to_value(ExecTicket {
        namespace,
        pod,
        container,
    })?;

    cache
        .set(&ticket_key(&ticket), value, Some(TICKET_TTL_SECS))
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(ticket)
}

/// Redeems (and consumes) a ticket minted by [`mint`]. Called directly from
/// `server/src/ws.rs`'s plain actix handler, which has its own `CacheStore`
/// handle.
pub async fn redeem(store: &CacheStore, ticket: &str) -> Option<ExecTicket> {
    let value = store.take(&ticket_key(ticket)).await.ok().flatten()?;
    serde_json::from_value(value).ok()
}

fn ticket_key(ticket: &str) -> String {
    format!("exec-ticket:{ticket}")
}
