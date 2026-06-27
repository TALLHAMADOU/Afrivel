//! Requêtes entrantes (désérialisation + validation) du module Auth.

use afrivel::{Result, Validate, ensure};
use serde::Deserialize;

/// Corps de `POST /auth/register`.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// E-mail (doit contenir `@`).
    pub email: String,
    /// Mot de passe en clair (min. 8 caractères).
    pub password: String,
}

impl Validate for RegisterRequest {
    fn validate(&self) -> Result<()> {
        ensure(self.email.contains('@'), "email invalide")?;
        ensure(self.password.len() >= 8, "mot de passe trop court (min. 8)")?;
        Ok(())
    }
}

/// Corps de `POST /auth/login`.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// E-mail.
    pub email: String,
    /// Mot de passe en clair.
    pub password: String,
}

impl Validate for LoginRequest {
    fn validate(&self) -> Result<()> {
        ensure(
            !self.email.is_empty() && !self.password.is_empty(),
            "email et mot de passe requis",
        )
    }
}
