//! Chargement de configuration typée (TOML + surcharges d'environnement).

use std::path::Path;

use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

/// Préfixe des variables d'environnement reconnues (`AFRIVEL_…`).
pub const ENV_PREFIX: &str = "AFRIVEL_";

/// Charge la configuration depuis `config/app.toml`, surchargée par les variables
/// d'environnement préfixées `AFRIVEL_`.
pub fn load<T: DeserializeOwned>() -> Result<T> {
    load_from("config/app.toml")
}

/// Comme [`load`], mais en lisant le fichier TOML indiqué.
pub fn load_from<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    Figment::new()
        .merge(Toml::file(path.as_ref()))
        .merge(Env::prefixed(ENV_PREFIX))
        .extract()
        .map_err(|err| Error::Internal(format!("config: {err}")))
}
