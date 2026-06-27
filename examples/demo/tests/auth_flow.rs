//! Test d'intégration HTTP du module Auth : **inscription → connexion → accès protégé**.
//!
//! Monte l'`Application` avec le dépôt en mémoire (aucune base de données) et exerce les
//! routes via `tower::ServiceExt::oneshot`. C'est le critère de sortie du module M4.

use std::sync::Arc;

use afrivel::Application;
use afrivel::auth::jwt::JwtSecret;
use afrivel::axum::Router;
use afrivel::axum::body::Body;
use afrivel::axum::http::{Request, StatusCode, header};
use demo::auth::infra::memory::InMemoryUserRepository;
use demo::auth::{AuthModule, AuthService, UserRepository};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Construit le routeur de test et renvoie aussi le dépôt (pour les helpers `grant`).
fn harness() -> (Router, Arc<InMemoryUserRepository>) {
    let repo = Arc::new(InMemoryUserRepository::new());
    let users: Arc<dyn UserRepository> = repo.clone();
    let secret = JwtSecret::new("test-secret");
    let service = AuthService::new(users, secret.clone());
    let router = Application::new()
        .register(AuthModule)
        .provide(secret)
        .provide(service)
        .into_router();
    (router, repo)
}

/// POST JSON et renvoie (statut, corps JSON).
async fn post_json(router: &Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    send(router, req).await
}

/// GET avec un éventuel jeton Bearer.
async fn get(router: &Router, uri: &str, token: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method("GET").uri(uri);
    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    send(router, builder.body(Body::empty()).unwrap()).await
}

async fn send(router: &Router, req: Request<Body>) -> (StatusCode, Value) {
    let res = router.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, json)
}

#[tokio::test]
async fn register_login_then_access_protected_route() {
    let (router, _repo) = harness();

    // 1. Inscription → 201 + ressource utilisateur.
    let (status, body) = post_json(
        &router,
        "/auth/register",
        json!({ "email": "alice@example.com", "password": "supersecret" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "register: {body}");
    assert_eq!(body["email"], "alice@example.com");

    // 2. Connexion → 200 + jeton Bearer.
    let (status, body) = post_json(
        &router,
        "/auth/login",
        json!({ "email": "alice@example.com", "password": "supersecret" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "login: {body}");
    let token = body["token"].as_str().expect("token").to_string();

    // 3. Route protégée avec jeton → 200 + profil.
    let (status, body) = get(&router, "/auth/me", Some(&token)).await;
    assert_eq!(status, StatusCode::OK, "me: {body}");
    assert!(body["id"].is_string());

    // 4. Route protégée sans jeton → 401.
    let (status, _) = get(&router, "/auth/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn admin_route_enforces_role() {
    let (router, repo) = harness();
    let email = "boss@example.com";

    post_json(
        &router,
        "/auth/register",
        json!({ "email": email, "password": "supersecret" }),
    )
    .await;

    // Sans le rôle admin : le jeton est valide mais l'accès est refusé (403).
    let (_, body) = post_json(
        &router,
        "/auth/login",
        json!({ "email": email, "password": "supersecret" }),
    )
    .await;
    let token = body["token"].as_str().unwrap().to_string();
    let (status, _) = get(&router, "/auth/admin", Some(&token)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Après attribution du rôle admin, un nouveau jeton ouvre l'accès (200).
    repo.grant(email, &["admin"], &[]);
    let (_, body) = post_json(
        &router,
        "/auth/login",
        json!({ "email": email, "password": "supersecret" }),
    )
    .await;
    let token = body["token"].as_str().unwrap().to_string();
    let (status, body) = get(&router, "/auth/admin", Some(&token)).await;
    assert_eq!(status, StatusCode::OK, "admin: {body}");
    assert_eq!(body["roles"][0], "admin");
}

#[tokio::test]
async fn rejects_invalid_payloads() {
    let (router, _repo) = harness();

    // Mot de passe trop court → 422 (validation).
    let (status, _) = post_json(
        &router,
        "/auth/register",
        json!({ "email": "bob@example.com", "password": "short" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Identifiants inconnus → 401 (pas d'oracle d'énumération).
    let (status, _) = post_json(
        &router,
        "/auth/login",
        json!({ "email": "ghost@example.com", "password": "supersecret" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
