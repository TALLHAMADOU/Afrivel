//! Type d'erreur unifié du framework et son mapping vers une réponse HTTP.

use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;

/// Alias de `Result` utilisé dans tout Afrivel.
pub type Result<T> = std::result::Result<T, Error>;

/// Erreur applicative d'Afrivel.
///
/// Chaque variante porte un statut HTTP et un code stable, exposés via [`Error::status`]
/// et [`Error::code`], et sérialisés par l'implémentation d'[`IntoResponse`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Échec de validation d'une requête (HTTP 422).
    #[error("validation failed: {0}")]
    Validation(String),

    /// Ressource introuvable (HTTP 404).
    #[error("resource not found")]
    NotFound,

    /// Authentification requise ou invalide (HTTP 401).
    #[error("unauthorized")]
    Unauthorized,

    /// Accès refusé malgré une authentification valide (HTTP 403).
    #[error("forbidden")]
    Forbidden,

    /// Erreur provenant de la couche de persistance (HTTP 500).
    #[error("database error: {0}")]
    Database(String),

    /// Erreur interne non catégorisée (HTTP 500).
    #[error("internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Statut HTTP associé à l'erreur.
    pub fn status(&self) -> StatusCode {
        match self {
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::Database(_) | Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Code machine stable, destiné aux clients d'API.
    pub fn code(&self) -> &'static str {
        match self {
            Error::Validation(_) => "validation_error",
            Error::NotFound => "not_found",
            Error::Unauthorized => "unauthorized",
            Error::Forbidden => "forbidden",
            Error::Database(_) => "database_error",
            Error::Internal(_) => "internal_error",
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Les erreurs internes/BDD ne fuitent pas leur détail au client.
        let message = match &self {
            Error::Database(_) | Error::Internal(_) => "internal server error".to_string(),
            other => other.to_string(),
        };
        let body = Json(json!({
            "error": { "code": self.code(), "message": message }
        }));
        (self.status(), body).into_response()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
