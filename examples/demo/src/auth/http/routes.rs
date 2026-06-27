//! Table de routage du module Auth.

use afrivel::axum::Router;
use afrivel::axum::routing::{get, post};

use crate::auth::http::controllers;

/// Routes exposées par le module : inscription, connexion, profil, route admin (RBAC).
pub fn routes() -> Router {
    Router::new()
        .route("/auth/register", post(controllers::register))
        .route("/auth/login", post(controllers::login))
        .route("/auth/me", get(controllers::me))
        .route("/auth/admin", get(controllers::admin))
}
