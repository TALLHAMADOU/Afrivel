//! Émission et validation de **JWT** (HS256) : claims porteurs de l'identité, des rôles
//! et des permissions, signés avec un secret partagé.
//!
//! Le secret est injecté dans l'application via [`JwtSecret`] (typiquement
//! `Application::provide(JwtSecret::new(...))`) ; les extracteurs d'auth le lisent depuis
//! les extensions de requête.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Secret de signature partagé, clonable et injectable comme dépendance (`Extension`).
#[derive(Clone)]
pub struct JwtSecret(Arc<Vec<u8>>);

impl JwtSecret {
    /// Construit un secret à partir de n'importe quel porteur d'octets (chaîne, env…).
    pub fn new(secret: impl Into<Vec<u8>>) -> Self {
        Self(Arc::new(secret.into()))
    }

    fn bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

/// Charge utile d'un JWT Afrivel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// Sujet : identifiant de l'utilisateur (sérialisé en chaîne).
    pub sub: String,
    /// Expiration (timestamp Unix, secondes).
    pub exp: i64,
    /// Émission (timestamp Unix, secondes).
    pub iat: i64,
    /// Rôles accordés.
    #[serde(default)]
    pub roles: Vec<String>,
    /// Permissions accordées.
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl Claims {
    /// Construit des claims pour `subject`, expirant dans `ttl`.
    pub fn new(subject: impl Into<String>, ttl: Duration) -> Self {
        let now = unix_now();
        Self {
            sub: subject.into(),
            iat: now,
            exp: now + ttl.as_secs() as i64,
            roles: Vec::new(),
            permissions: Vec::new(),
        }
    }

    /// Ajoute des rôles (chaînage).
    pub fn with_roles(mut self, roles: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.roles = roles.into_iter().map(Into::into).collect();
        self
    }

    /// Ajoute des permissions (chaînage).
    pub fn with_permissions(
        mut self,
        permissions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.permissions = permissions.into_iter().map(Into::into).collect();
        self
    }
}

/// Signe `claims` en HS256 et renvoie le jeton compact.
pub fn encode(claims: &Claims, secret: &JwtSecret) -> Result<String> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.bytes()),
    )
    .map_err(|e| Error::Internal(format!("jwt encode: {e}")))
}

/// Vérifie la signature et l'expiration de `token`, puis renvoie ses claims.
///
/// Toute défaillance (signature, expiration, format) donne [`Error::Unauthorized`] —
/// aucun détail interne n'est exposé au client.
pub fn decode(token: &str, secret: &JwtSecret) -> Result<Claims> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| Error::Unauthorized)
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secret() -> JwtSecret {
        JwtSecret::new("test-secret")
    }

    #[test]
    fn encode_then_decode_roundtrip() {
        let claims = Claims::new("42", Duration::from_secs(3600))
            .with_roles(["admin"])
            .with_permissions(["posts.write"]);
        let token = encode(&claims, &secret()).expect("encode");
        let back = decode(&token, &secret()).expect("decode");
        assert_eq!(back.sub, "42");
        assert_eq!(back.roles, vec!["admin"]);
        assert_eq!(back.permissions, vec!["posts.write"]);
    }

    #[test]
    fn wrong_secret_is_unauthorized() {
        let token = encode(&Claims::new("1", Duration::from_secs(60)), &secret()).unwrap();
        let err = decode(&token, &JwtSecret::new("other")).unwrap_err();
        assert!(matches!(err, Error::Unauthorized));
    }

    #[test]
    fn expired_token_is_unauthorized() {
        // ttl nul → exp == iat == maintenant ; la marge `leeway` par défaut de
        // jsonwebtoken est de 60s, donc on force une expiration franchement passée.
        let mut claims = Claims::new("1", Duration::from_secs(0));
        claims.exp = unix_now() - 3600;
        let token = encode(&claims, &secret()).unwrap();
        assert!(matches!(
            decode(&token, &secret()),
            Err(Error::Unauthorized)
        ));
    }
}
