
use crate::dto::booking_response::{BookingResponse, Ticket};

pub fn booking_response(order_id: &str, tickets: Vec<Ticket>) -> BookingResponse {
    BookingResponse {
        order_id: order_id.into(),
        cancellation_id: Some(order_id.into()),
        tickets,
        booking_amount: None,
        pickup_hotel: None,
        pickup_location: None,
        pickup_time: None,
        voucher_display_info: None,
    }
}