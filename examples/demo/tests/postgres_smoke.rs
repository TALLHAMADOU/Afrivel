//! Smoke test **end-to-end sur Postgres** : applique les migrations puis exerce le flux
//! HTTP inscription → connexion → accès protégé contre une vraie base.
//!
//! Activé uniquement lorsque `DATABASE_URL` est défini (job `integration` de la CI). En
//! l'absence de variable — par ex. le job `check` sans base — le test se neutralise et passe.

use afrivel::auth::jwt::JwtSecret;
use afrivel::axum::Router;
use afrivel::axum::body::Body;
use afrivel::axum::http::{Request, StatusCode, header};
use afrivel::orm::sea_orm::Database;
use afrivel::orm::sea_orm_migration::MigratorTrait;
use demo::migrator::Migrator;
use demo::registry;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn post_json(router: &Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    send(router, req).await
}

async fn send(router: &Router, req: Request<Body>) -> (StatusCode, Value) {
    let res = router.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

#[tokio::test]
async fn end_to_end_flow_on_postgres() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL absent — smoke Postgres ignoré.");
        return;
    };

    let db = Database::connect(url).await.expect("connexion Postgres");
    // Base propre : (re)crée tout le schéma du module Auth.
    Migrator::fresh(&db).await.expect("migrate:fresh");

    let secret = JwtSecret::new("smoke-secret");
    let router = registry::application(db, secret).into_router();

    // Inscription → 201.
    let (status, body) = post_json(
        &router,
        "/auth/register",
        json!({ "email": "smoke@example.com", "password": "supersecret" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "register: {body}");

    // Connexion → 200 + jeton.
    let (status, body) = post_json(
        &router,
        "/auth/login",
        json!({ "email": "smoke@example.com", "password": "supersecret" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "login: {body}");
    let token = body["token"].as_str().expect("token").to_string();

    // Route protégée avec jeton → 200.
    let req = Request::builder()
        .method("GET")
        .uri("/auth/me")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let (status, body) = send(&router, req).await;
    assert_eq!(status, StatusCode::OK, "me: {body}");

    // Route protégée sans jeton → 401.
    let req = Request::builder()
        .method("GET")
        .uri("/auth/me")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send(&router, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
