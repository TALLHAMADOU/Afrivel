//! Primitives d'authentification et d'autorisation du framework.
//!
//! - [`hashing`] : hachage de mots de passe (Argon2).
//! - [`jwt`] : émission/validation de jetons (claims, rôles, permissions).
//! - [`AuthUser`] : extracteur Axum de l'utilisateur authentifié (depuis le `Bearer` JWT).
//! - [`authenticate`] : middleware protégeant un groupe de routes.
//! - [`Guard`] / [`Authorized`] : autorisation **RBAC** par extracteur typé.
//!
//! Le secret de signature ([`jwt::JwtSecret`]) est fourni à l'application via
//! `Application::provide(secret)` ; extracteur et middleware le lisent dans les extensions.

pub mod hashing;
pub mod jwt;

use std::marker::PhantomData;

use axum::extract::{FromRequestParts, Request};
use axum::middleware::Next;
use axum::response::Response;
use http::HeaderMap;
use http::header::AUTHORIZATION;
use http::request::Parts;

use crate::error::{Error, Result};
use jwt::{Claims, JwtSecret};

/// Utilisateur authentifié, reconstruit depuis les claims d'un JWT valide.
///
/// S'extrait dans un handler (`async fn handler(user: AuthUser)`). Si [`authenticate`] a
/// déjà tourné sur la route, l'extracteur réutilise l'utilisateur stocké ; sinon il décode
/// lui-même l'en-tête `Authorization: Bearer <token>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthUser {
    /// Identifiant de l'utilisateur (claim `sub`).
    pub id: String,
    /// Rôles accordés.
    pub roles: Vec<String>,
    /// Permissions accordées.
    pub permissions: Vec<String>,
}

impl AuthUser {
    fn from_claims(claims: Claims) -> Self {
        Self {
            id: claims.sub,
            roles: claims.roles,
            permissions: claims.permissions,
        }
    }

    /// Vrai si l'utilisateur possède `role`.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Vrai si l'utilisateur possède `permission`.
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    /// `Ok` si l'utilisateur possède `role`, sinon [`Error::Forbidden`].
    pub fn require_role(&self, role: &str) -> Result<()> {
        if self.has_role(role) {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    /// `Ok` si l'utilisateur possède `permission`, sinon [`Error::Forbidden`].
    pub fn require_permission(&self, permission: &str) -> Result<()> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // Déjà authentifié par le middleware `authenticate` ?
        if let Some(user) = parts.extensions.get::<AuthUser>() {
            return Ok(user.clone());
        }
        let secret = parts
            .extensions
            .get::<JwtSecret>()
            .ok_or_else(|| Error::Internal("JwtSecret non fourni à l'application".into()))?;
        let token = bearer_token(&parts.headers).ok_or(Error::Unauthorized)?;
        let claims = jwt::decode(token, secret)?;
        Ok(AuthUser::from_claims(claims))
    }
}

/// Middleware d'authentification : exige un JWT valide, sinon répond `401`.
///
/// En cas de succès, insère l'[`AuthUser`] dans les extensions de la requête pour que les
/// handlers en aval l'extraient sans redécoder le jeton. À brancher via
/// `axum::middleware::from_fn` sur le sous-routeur à protéger.
pub async fn authenticate(mut req: Request, next: Next) -> Result<Response> {
    let secret = req
        .extensions()
        .get::<JwtSecret>()
        .cloned()
        .ok_or_else(|| Error::Internal("JwtSecret non fourni à l'application".into()))?;
    let token = bearer_token(req.headers()).ok_or(Error::Unauthorized)?;
    let claims = jwt::decode(token, &secret)?;
    req.extensions_mut().insert(AuthUser::from_claims(claims));
    Ok(next.run(req).await)
}

/// Règle d'autorisation **RBAC**, vérifiée à l'extraction via [`Authorized`].
///
/// Implémentez ce trait sur un type marqueur pour exiger un rôle/permission au niveau du
/// handler :
///
/// ```ignore
/// struct Admin;
/// impl afrivel::auth::Guard for Admin {
///     fn authorize(user: &afrivel::auth::AuthUser) -> afrivel::Result<()> {
///         user.require_role("admin")
///     }
/// }
/// async fn dashboard(_: Authorized<Admin>) { /* réservé aux admins */ }
/// ```
pub trait Guard {
    /// Autorise (ou non) `user` ; renvoyer [`Error::Forbidden`] pour refuser.
    fn authorize(user: &AuthUser) -> Result<()>;
}

/// Extracteur combinant authentification ([`AuthUser`]) et autorisation ([`Guard`]).
///
/// Échoue en `401` si l'utilisateur n'est pas authentifié, en `403` si la garde refuse.
pub struct Authorized<G: Guard> {
    /// L'utilisateur authentifié et autorisé.
    pub user: AuthUser,
    _guard: PhantomData<G>,
}

impl<G: Guard, S: Send + Sync> FromRequestParts<S> for Authorized<G> {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        G::authorize(&user)?;
        Ok(Authorized {
            user,
            _guard: PhantomData,
        })
    }
}

/// Extrait le jeton d'un en-tête `Authorization: Bearer <token>`.
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let token = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))?;
    let token = token.trim();
    (!token.is_empty()).then_some(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Application;
    use crate::app::Module;
    use axum::Router;
    use axum::routing::get;
    use http::{Request, StatusCode};
    use std::time::Duration;
    use tower::ServiceExt;

    fn guard_checks() {
        let user = AuthUser {
            id: "1".into(),
            roles: vec!["editor".into()],
            permissions: vec!["posts.write".into()],
        };
        assert!(user.has_role("editor"));
        assert!(!user.has_role("admin"));
        assert!(user.require_role("editor").is_ok());
        assert!(matches!(user.require_role("admin"), Err(Error::Forbidden)));
        assert!(user.require_permission("posts.write").is_ok());
    }

    #[test]
    fn auth_user_role_and_permission_helpers() {
        guard_checks();
    }

    struct AuthOnly;
    impl Module for AuthOnly {
        fn name(&self) -> &'static str {
            "t"
        }
        fn routes(&self) -> Router {
            Router::new().route("/me", get(|user: AuthUser| async move { user.id }))
        }
    }

    async fn router() -> Router {
        Application::new()
            .register(AuthOnly)
            .provide(JwtSecret::new("k"))
            .into_router()
    }

    #[tokio::test]
    async fn extractor_rejects_missing_token() {
        let res = router()
            .await
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn extractor_accepts_valid_token() {
        let token = jwt::encode(
            &Claims::new("99", Duration::from_secs(60)),
            &JwtSecret::new("k"),
        )
        .unwrap();
        let res = router()
            .await
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header(AUTHORIZATION, format!("Bearer {token}"))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
