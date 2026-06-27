//! Service métier d'authentification : enregistrement et connexion.
//!
//! Dépend du **port** [`UserRepository`] (trait) et des primitives d'auth du noyau
//! (`afrivel::auth`). Aucune dépendance vers SeaORM ni vers le web → testable avec un
//! dépôt en mémoire.

use std::sync::Arc;
use std::time::Duration;

use afrivel::auth::hashing;
use afrivel::auth::jwt::{self, Claims, JwtSecret};
use afrivel::{Error, Result};

use crate::auth::contracts::{UserRef, UserRepository};

/// Durée de validité par défaut d'un jeton émis (24 h).
const TOKEN_TTL: Duration = Duration::from_secs(24 * 3600);

/// Service d'authentification, injecté dans les handlers via `Extension<AuthService>`.
#[derive(Clone)]
pub struct AuthService {
    users: Arc<dyn UserRepository>,
    secret: JwtSecret,
    ttl: Duration,
}

impl AuthService {
    /// Câble le service sur un dépôt et un secret de signature.
    pub fn new(users: Arc<dyn UserRepository>, secret: JwtSecret) -> Self {
        Self {
            users,
            secret,
            ttl: TOKEN_TTL,
        }
    }

    /// Enregistre un nouvel utilisateur (mot de passe haché en Argon2).
    ///
    /// Renvoie [`Error::Validation`] si l'e-mail est déjà pris.
    pub async fn register(&self, email: &str, password: &str) -> Result<UserRef> {
        if self.users.by_email(email).await?.is_some() {
            return Err(Error::Validation("email déjà utilisé".into()));
        }
        let hash = hashing::hash_password(password)?;
        self.users.create(email, &hash).await
    }

    /// Vérifie les identifiants et renvoie un JWT signé (rôles/permissions inclus).
    ///
    /// Renvoie [`Error::Unauthorized`] si l'e-mail est inconnu ou le mot de passe faux —
    /// sans distinguer les deux cas (pas d'oracle d'énumération de comptes).
    pub async fn login(&self, email: &str, password: &str) -> Result<String> {
        let creds = self
            .users
            .by_email(email)
            .await?
            .ok_or(Error::Unauthorized)?;
        if !hashing::verify_password(password, &creds.password_hash) {
            return Err(Error::Unauthorized);
        }
        let claims = Claims::new(creds.id, self.ttl)
            .with_roles(creds.roles)
            .with_permissions(creds.permissions);
        jwt::encode(&claims, &self.secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::infra::memory::InMemoryUserRepository;

    fn service() -> AuthService {
        let users: Arc<dyn UserRepository> = Arc::new(InMemoryUserRepository::new());
        AuthService::new(users, JwtSecret::new("test-secret"))
    }

    #[tokio::test]
    async fn register_then_login_emits_a_token() {
        let svc = service();
        let user = svc.register("a@b.io", "supersecret").await.unwrap();
        assert_eq!(user.email, "a@b.io");
        let token = svc.login("a@b.io", "supersecret").await.unwrap();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn register_rejects_duplicate_email() {
        let svc = service();
        svc.register("a@b.io", "supersecret").await.unwrap();
        let err = svc.register("a@b.io", "supersecret").await.unwrap_err();
        assert!(matches!(err, Error::Validation(_)));
    }

    #[tokio::test]
    async fn login_rejects_wrong_password() {
        let svc = service();
        svc.register("a@b.io", "supersecret").await.unwrap();
        let err = svc.login("a@b.io", "wrongpass").await.unwrap_err();
        assert!(matches!(err, Error::Unauthorized));
    }

    #[tokio::test]
    async fn login_rejects_unknown_email() {
        let svc = service();
        let err = svc.login("ghost@b.io", "supersecret").await.unwrap_err();
        assert!(matches!(err, Error::Unauthorized));
    }
}
