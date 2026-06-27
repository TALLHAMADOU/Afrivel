//! Module **Auth** : inscription, connexion (JWT) et RBAC, en Clean Architecture.
//!
//! Couches : [`contracts`] (ports) → [`domain`] (entités + services) → [`http`] (adaptateurs)
//! et [`infra`] (implémentations des ports). Les autres modules ne voient que [`contracts`].

pub mod contracts;
pub mod domain;
pub mod guards;
pub mod http;
pub mod infra;
mod migration;

use afrivel::Module;
use afrivel::axum::Router;
use afrivel::orm::sea_orm_migration::MigrationTrait;

pub use contracts::{Credentials, UserRef, UserRepository};
pub use domain::service::AuthService;

/// Le module Auth (implémente le contrat [`Module`] du noyau).
pub struct AuthModule;

impl Module for AuthModule {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn routes(&self) -> Router {
        http::routes::routes()
    }
}

/// Constructeur du module (symétrique des modules générés par la CLI).
pub fn module() -> AuthModule {
    AuthModule
}

/// Migrations possédées par le module (agrégées par le `migrator` de l'app).
pub fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(migration::Migration)]
}
