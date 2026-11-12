use axum::{Router, routing::get};

use crate::{controller, state::AppState};

/// Builds the application router. Kept separate from `main` so tests can drive it directly.
pub fn router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .merge(controller::booking_controller_v2::routes())
        .with_state(AppState::new())
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
