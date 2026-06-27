//! Ressources sortantes (DTO de réponse) du module Auth.

use serde::Serialize;

use crate::auth::contracts::UserRef;

/// Représentation publique d'un utilisateur.
#[derive(Debug, Serialize)]
pub struct UserResource {
    pub id: String,
    pub email: String,
}

impl From<UserRef> for UserResource {
    fn from(user: UserRef) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

/// Réponse d'émission de jeton.
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: &'static str,
}

impl TokenResponse {
    /// Construit une réponse `Bearer`.
    pub fn bearer(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer",
        }
    }
}

/// Profil de l'utilisateur authentifié (route protégée `/auth/me`).
#[derive(Debug, Serialize)]
pub struct ProfileResource {
    pub id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
