//! Background listeners for domain events. Spring's `@EventListener` beans, wired by hand.
//!
//! Each listener subscribes to the bus and runs in its own tokio task. Call
//! [`spawn_all`] once at startup, inside a tokio runtime.

use crate::event_bus::{EventBus, RecvError};
use tokio::task::JoinHandle;

use crate::event::BookingCreatedEvent;

/// Starts every listener on `bus`.
pub fn spawn_all(bus: &EventBus) -> Vec<JoinHandle<()>> {
    vec![spawn_booking_created_logger(bus)]
}

/// Logs every [`BookingCreatedEvent`] at `info` level.
///
/// Subscribes before returning, so events published after this call are not missed.
/// If the listener falls behind and the bus drops events, it logs a warning and keeps
/// going. The task ends only when the bus is dropped.
pub fn spawn_booking_created_logger(bus: &EventBus) -> JoinHandle<()> {
    let mut rx = bus.subscribe::<BookingCreatedEvent>();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(event) => log_booking_created(&event),
                Err(RecvError::Lagged(missed)) => {
                    tracing::warn!(missed, "booking-created logger lagged, events dropped");
                }
                Err(RecvError::Closed) => break,
            }
        }
    })
}

fn log_booking_created(event: &BookingCreatedEvent) {
    tracing::info!(
        booking_id = event.booking_id,
        itinerary_id = event.itinerary_id,
        rms = %event.rms_account_name,
        product_code = %event.booking.product_code,
        inventory = %event.booking.inventory_date.and_time(event.booking.inventory_time),
        order_id = %event.response.order_id,
        tickets = event.response.tickets.len(),
        "booking created"
    );
}
