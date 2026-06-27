//! Contrôleurs HTTP (handlers Axum) du module Auth — adaptateurs vers [`AuthService`].

use afrivel::auth::{AuthUser, Authorized};
use afrivel::axum::http::StatusCode;
use afrivel::axum::{Extension, Json};
use afrivel::{Result, ValidatedJson};

use crate::auth::domain::service::AuthService;
use crate::auth::guards::AdminGuard;
use crate::auth::http::requests::{LoginRequest, RegisterRequest};
use crate::auth::http::resources::{ProfileResource, TokenResponse, UserResource};

/// `POST /auth/register` — crée un compte, renvoie `201` + la ressource utilisateur.
pub async fn register(
    Extension(service): Extension<AuthService>,
    ValidatedJson(req): ValidatedJson<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResource>)> {
    let user = service.register(&req.email, &req.password).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

/// `POST /auth/login` — vérifie les identifiants, renvoie un JWT `Bearer`.
pub async fn login(
    Extension(service): Extension<AuthService>,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<Json<TokenResponse>> {
    let token = service.login(&req.email, &req.password).await?;
    Ok(Json(TokenResponse::bearer(token)))
}

/// `GET /auth/me` — route protégée : renvoie le profil de l'utilisateur authentifié.
pub async fn me(user: AuthUser) -> Json<ProfileResource> {
    Json(ProfileResource {
        id: user.id,
        roles: user.roles,
        permissions: user.permissions,
    })
}

/// `GET /auth/admin` — route protégée **RBAC** : réservée au rôle `admin`.
pub async fn admin(guard: Authorized<AdminGuard>) -> Json<ProfileResource> {
    let user = guard.user;
    Json(ProfileResource {
        id: user.id,
        roles: user.roles,
        permissions: user.permissions,
    })
}
