//! Registre de compilation : câble le module Auth et ses dépendances dans l'`Application`.
//!
//! Symétrique du `registry.rs` généré par la CLI ; ici on injecte en plus le dépôt SeaORM,
//! le service d'auth et le secret JWT (DI explicite via `Application::provide`).

use std::sync::Arc;

use afrivel::Application;
use afrivel::auth::jwt::JwtSecret;
use afrivel::orm::sea_orm::DatabaseConnection;

use crate::auth::infra::seaorm::SeaOrmUserRepository;
use crate::auth::{AuthModule, AuthService, UserRepository};

/// Construit l'application : module Auth + DI (connexion, secret, service).
pub fn application(db: DatabaseConnection, secret: JwtSecret) -> Application {
    let users: Arc<dyn UserRepository> = Arc::new(SeaOrmUserRepository::new(db.clone()));
    let service = AuthService::new(users, secret.clone());
    Application::new()
        .register(AuthModule)
        .provide(db)
        .provide(secret)
        .provide(service)
}
