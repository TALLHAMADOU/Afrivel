//! Validation des requêtes entrantes.

use axum::Json;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

/// Contrat de validation implémenté par les types de requête.
///
/// Renvoie [`Error::Validation`] lorsque les données ne respectent pas les règles métier.
pub trait Validate {
    /// Valide les données ; `Ok(())` si elles sont conformes.
    fn validate(&self) -> Result<()>;
}

/// Aide à la validation : renvoie [`Error::Validation`] si `condition` est fausse.
pub fn ensure(condition: bool, message: impl Into<String>) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(Error::Validation(message.into()))
    }
}

/// Extracteur Axum qui désérialise un corps JSON puis exécute [`Validate::validate`].
///
/// Combine désérialisation et validation : un échec de l'un comme de l'autre produit une
/// réponse HTTP 422 normalisée.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| Error::Validation(err.to_string()))?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}
