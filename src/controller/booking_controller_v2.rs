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
// #[axum::debug_handler]
async fn book_v2(
    State(state): State<AppState>,
    Query(params): Query<BookParams>,
    Json(booking_request): Json<AMEnduranceBookingRequest>,
) -> Result<Json<Option<BookingResponse>>, AppError> {
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
