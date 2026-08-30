//! Notifications mount host: `/notifications` session gate + mount contract names.
//!
//! Mirrors what a product host does before mounting
//! [`uf_notifications::NotificationsRoutes`] and
//! [`uf_notifications::NotificationBell`]: deny anonymous `/notifications`, and
//! return the same `uf_app!` id/path plus shell-bell / send / Photon WS names
//! a host wires next.
//!
//! This binary is an Axum oneshot (no Leptos SSR/WASM, no `uf-product` link).
//! Copy the product mount `Cargo.toml` feature graph and the Leptos sketch from
//! the host README into a real Unified Field product binary.
//!
//! ## When to use
//! Smoke the notifications product mount contract (path + auth + discovery
//! names) without compiling the Orbital/Leptos UI graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-uf-notifications
//! cargo run -p notifications-mount-host
//! ```
//!
//! ## Success
//! Stdout prints `notifications_mount_host: OK — /notifications protect + inventory + bell slot`.
//!
//! ## Look next
//! Mount `NotificationsRoutes` at `/notifications`, provide `NotificationBell`
//! via `provide_shell_notification_bell`, and call `send_notification` from
//! backend code. Photon live unread uses topic `user.notifications` over
//! `/ws/notifications`.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Mirror of `uf-notifications` `uf_app!` id / route_path (see UI crate `lib.rs`).
const APP_ID: &str = "notifications";
const ROUTE_PATH: &str = "/notifications";

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn notifications_surface(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    Json(serde_json::json!({
        "path": ROUTE_PATH,
        "app_id": APP_ID,
        "user": session.user_id,
        "shell_notification_bell": "NotificationBell",
        "send_api": "uf_notifications_core::send_notification",
        "photon_ws": "/ws/notifications",
    }))
}

fn app() -> Router {
    Router::new()
        .route(ROUTE_PATH, get(notifications_surface))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    assert_eq!(status_for(ROUTE_PATH, None).await, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri(ROUTE_PATH)
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], ROUTE_PATH);
    assert_eq!(body["app_id"], APP_ID);
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["shell_notification_bell"], "NotificationBell");
    assert_eq!(body["send_api"], "uf_notifications_core::send_notification");
    assert_eq!(body["photon_ws"], "/ws/notifications");

    println!("notifications_mount_host: OK — /notifications protect + inventory + bell slot");
}
