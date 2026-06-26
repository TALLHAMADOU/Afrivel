//! Noyau d'Afrivel : routing (Axum/Tower), type d'erreur unifié (`Error` → `IntoResponse`),
//! configuration typée, logging (`tracing`), validation et injection de dépendances.
#![forbid(unsafe_code)]

pub mod app;
pub mod config;
pub mod error;
pub mod logging;
pub mod validation;

pub use app::{Application, Module};
pub use error::{Error, Result};
pub use validation::{Validate, ValidatedJson, ensure};

/// Re-export d'`axum` : les modules générés construisent leur `Router` avec **cette**
/// version, ce qui élimine toute dérive de version entre le noyau et les modules.
pub use axum;
/// Re-export de `tokio` pour le binaire `app` généré (runtime + `#[tokio::main]`).
pub use tokio;

/// Re-exports les plus courants pour les applications et modules générés.
pub mod prelude {
    pub use crate::app::{Application, Module};
    pub use crate::error::{Error, Result};
    pub use crate::validation::{Validate, ValidatedJson, ensure};
}
