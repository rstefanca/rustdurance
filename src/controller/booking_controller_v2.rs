use axum::{
    Json, Router,
    extract::{Query, State},
    routing::post,
};
use serde::Deserialize;

use crate::{
    dto::{booking_request::AMEnduranceBookingRequest, booking_response::BookingResponse},
    error::AppError,
    state::AppState,
};
use crate::error::AppError::NotImplemented;

/// Port of Endurance's `BookingControllerV2` (`@RequestMapping("/v2/booking")`).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v2/booking", post(book_v2))
        .route("/v2/cancel", post(cancel))
}

#[derive(Debug, Deserialize)]
struct BookParams {
    #[serde(default)]
    preflight: bool,
}

/// `POST /v2/booking?preflight=false`
// TODO: `@PreAuthorize("hasAnyAuthority('OP_BOOKING_WRITE') || hasRole('ADMIN')")` once auth exists.
// `priorityDispatcher.withMaxPrioDispatcher` has no equivalent: tokio schedules the handler itself.
async fn book_v2(
    State(state): State<AppState>,
    Query(params): Query<BookParams>,
    Json(booking_request): Json<AMEnduranceBookingRequest>,
) -> Result<Json<BookingResponse>, AppError> {
    let result = state
        .booking_service
        .book(params.preflight, booking_request)
        .await?;
    Ok(Json(result))
}

async fn cancel(
    State(s): State<AppState>,
) -> Result<(), AppError> {
    Err(NotImplemented("Cancel"))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, header::CONTENT_TYPE},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use crate::app::router;

    fn valid_request() -> Value {
        json!({
            "bookingId": 123,
            "itineraryId": 123,
            "tourId": 123,
            "tourCurrency": "USD",
            "supplyPartnerId": 1,
            "supplyPartnerName": "TEST_SUPPLY_PARTNER_NAME",
            "rmsAccountName": "DUMMY",
            "fulfilmentProcess": "DIRECT",
            "enduranceVendorBooking": {
                "bookingId": 123,
                "vendorId": 1,
                "tourId": 123,
                "inventoryDate": "2026-11-08",
                "inventoryTime": "00:00:00",
                "productName": "TEST_PRODUCT_NAME",
                "productCode": "{\"id\":123}",
                "primaryGuest": {
                    "firstName": "Test",
                    "lastName": "Adult",
                    "email": "test_adult@headout.com",
                    "phoneNumber": "123456789"
                },
                "guestNumbersMap": { "ADULT": 1 },
                "thresholdPriceUSD": { "value": 12.5, "currency": "USD" }
            }
        })
    }

    async fn post_booking(uri: &str, body: Value) -> StatusCode {
        post_booking_response(uri, body).await.status()
    }

    async fn post_booking_response(uri: &str, body: Value) -> axum::response::Response {
        router()
            .oneshot(
                Request::post(uri)
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn valid_request_returns_the_stubbed_response() {
        let response = post_booking_response("/v2/booking", valid_request()).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["orderId"], "1");
    }

    #[tokio::test]
    async fn preflight_flag_is_accepted() {
        assert_eq!(
            post_booking("/v2/booking?preflight=true", valid_request()).await,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn invalid_preflight_is_a_bad_request() {
        assert_eq!(
            post_booking("/v2/booking?preflight=maybe", valid_request()).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn incomplete_body_is_rejected() {
        assert_eq!(
            post_booking("/v2/booking", json!({ "bookingId": 1 })).await,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}
