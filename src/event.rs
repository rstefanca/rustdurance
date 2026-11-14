//! Domain events published on the application's [`crate::event_bus::EventBus`].
//!
//! Subscribe with `state.event_bus.subscribe::<BookingCreatedEvent>()`.

use crate::dto::booking_request::EnduranceVendorBooking;
use crate::dto::booking_response::BookingResponse;

/// A plugin booked successfully. Not published for preflight requests or failed bookings.
#[derive(Debug, Clone)]
pub struct BookingCreatedEvent {
    pub booking_id: i32,
    pub itinerary_id: i64,
    pub rms_account_name: String,
    pub booking: EnduranceVendorBooking,
    pub response: BookingResponse,
}
