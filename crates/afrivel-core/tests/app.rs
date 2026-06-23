//! Tests d'intégration du noyau : composition d'application, route de santé, mapping d'erreur.

use afrivel_core::prelude::*;
use axum::Router;
use axum::body::Body;
use axum::routing::get;
use http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Module de test minimal : `/health` (200) et `/missing` (renvoie `Error::NotFound`).
struct HealthModule;

impl Module for HealthModule {
    fn name(&self) -> &'static str {
        "health"
    }

    fn routes(&self) -> Router {
        Router::new()
            .route("/health", get(|| async { "ok" }))
            .route(
                "/missing",
                get(|| async { Result::<&'static str>::Err(Error::NotFound) }),
            )
    }
}

#[tokio::test]
async fn health_route_returns_200() {
    let router = Application::new().register(HealthModule).into_router();

    let response = router
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn error_maps_to_normalised_json() {
    let router = Application::new().register(HealthModule).into_router();

    let response = router
        .oneshot(Request::get("/missing").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["error"]["code"], "not_found");
}

#[test]
fn modules_are_tracked_in_order() {
    let app = Application::new().register(HealthModule);
    assert_eq!(app.modules(), &["health"]);
}
