//! Initialisation du logging structuré (`tracing`).

use tracing_subscriber::EnvFilter;

/// Initialise l'abonné `tracing` global (format texte + filtre par `RUST_LOG`).
///
/// Sans `RUST_LOG`, le niveau par défaut est `info`. Idempotent : un second appel est sans
/// effet (utile en tests).
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
