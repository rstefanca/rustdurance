use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;

use crate::{controller, state::AppState};

/// Builds the application router with fresh state. Kept separate from `main` so tests
/// can drive it directly.
#[cfg(test)]
pub fn router() -> Router {
    router_with(AppState::new())
}

/// Builds the application router around existing state, so `main` can start event
/// listeners on `state.event_bus` first.
pub fn router_with(state: AppState) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .merge(controller::booking_controller_v2::routes())
        .layer(CatchPanicLayer::new())
        .with_state(state)
}

async fn ping() -> &'static str {
    "pong"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn ping_returns_pong() {
        let response = router()
            .oneshot(Request::get("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"pong");
    }

    #[tokio::test]
    async fn unknown_route_is_404() {
        let response = router()
            .oneshot(Request::get("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
