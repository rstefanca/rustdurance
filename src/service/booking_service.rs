use crate::dto::booking_request::{EnduranceVendorBooking, VendorGuest};
use crate::error::PluginError;
use crate::event::BookingCreatedEvent;
use crate::plugin::guest::Guest;
use crate::plugin::guests::Guests;
use crate::plugin::pax_type::PaxType;
use crate::plugin::plugin::FeatureKind::Booking;
use crate::plugin::plugin::{BookingRequest, EndurancePlugin};
pub(crate) use crate::service::plugin_service::PluginService;
use crate::{
    dto::{booking_request::AMEnduranceBookingRequest, booking_response::BookingResponse},
    error::AppError,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::event_bus::EventBus;
use std::sync::Arc;

/// Port of Endurance's `BookingService`. Only the entry point exists so far.
pub struct BookingService {
    plugin_service: PluginService,
    event_bus: EventBus,
}

impl BookingService {
    pub fn new(plugin_service: PluginService, event_bus: EventBus) -> Self {
        Self {
            plugin_service,
            event_bus,
        }
    }

    /// Kotlin: `bookingService.book(preflight, bookingRequest)`.
    // TODO: port the rest of the booking flow (retries, post-processing).
    pub async fn book(
        &self,
        preflight: bool,
        request: AMEnduranceBookingRequest,
    ) -> Result<Option<BookingResponse>, AppError> {
        let rms = &request.rms_account_name;
        if preflight {
            // check if booking is possible
            self.plugin_service
                .get_booking_with_feature(rms, &Booking)?;
            return Ok(None);
        }
        let plugin = self.resolve_plugin(&request.rms_account_name)?;
        let vendor_booking = request.endurance_vendor_booking;
        let br = &BookingRequestImpl::new(vendor_booking.clone());
        let response = plugin.book(br).await.map_err(lift_error)?;
        
        self.event_bus.publish(BookingCreatedEvent {
            booking_id: request.booking_id,
            itinerary_id: request.itinerary_id,
            rms_account_name: request.rms_account_name,
            booking: vendor_booking.clone(),
            response: response.clone(),
        });

        Ok(Some(response))
    }

    fn resolve_plugin(&self, rms: &str) -> Result<Arc<EndurancePlugin>, AppError> {
        self.plugin_service.resolve_plugin(rms)
    }
}

fn lift_error(plugin_error: PluginError) -> AppError {
    AppError::PluginError {
        cause: plugin_error,
    }
}

struct BookingRequestImpl {
    delegate: EnduranceVendorBooking,
    guests: Guests,
}

impl BookingRequestImpl {
    fn new(delegate: EnduranceVendorBooking) -> Self {
        let list = &mut delegate
            .guests
            .clone()
            .iter()
            .map(|g| Guest {
                first_name: "dfq".to_string(),
                last_name: "dfd".to_string(),
                pax_type: "Adult".into(),
                phone_number: None,
                email: None,
                custom_fields: Vec::new(),
            })
            .collect::<Vec<Guest>>();

       let pax_numbers = delegate.guest_numbers_map.iter()
           .map(|(k,v)| (PaxType::from(k.as_str()), *v))
           .collect();

        let guests = Guests::new(list, pax_numbers);
        Self { delegate, guests }
    }

    //delegate
    fn main_guest(&self) -> &Guest {
        self.guests.main_guest()
    }
}

impl BookingRequest for BookingRequestImpl {
    fn booking_id(&self) -> &u32 {
        &self.delegate.booking_id
    }

    fn product_code(&self) -> &String {
        &self.delegate.product_code
    }

    fn booking_date(&self) -> &NaiveDate {
        &self.delegate.inventory_date
    }

    fn booking_time(&self) -> &NaiveTime {
        &self.delegate.inventory_time
    }

    fn booking_date_time(&self) -> NaiveDateTime {
        self.delegate
            .inventory_date
            .and_time(self.delegate.inventory_time)
    }

    fn guests(&self) -> &Guests {
        &self.guests
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;

    fn request(rms: &str) -> AMEnduranceBookingRequest {
        serde_json::from_value(serde_json::json!({
            "bookingId": 42,
            "itineraryId": 4242,
            "tourId": 1,
            "tourCurrency": "EUR",
            "supplyPartnerId": 1,
            "supplyPartnerName": "partner",
            "rmsAccountName": rms,
            "enduranceVendorBooking": {
                "bookingId": 42,
                "vendorId": 1,
                "tourId": 1,
                "inventoryDate": "2026-12-01",
                "inventoryTime": "10:00:00",
                "productName": "product",
                "productCode": "{}",
                "primaryGuest": { "firstName": "Jan", "lastName": "Novak" },
                "guestNumbersMap": { "ADULT": 2 },
                "guests": [{}]
            }
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn successful_booking_publishes_event() {
        let state = AppState::new();
        let mut rx = state.event_bus.subscribe::<BookingCreatedEvent>();

        let response = state
            .booking_service
            .book(false, request("DUMMY"))
            .await
            .unwrap()
            .unwrap();

        let event = rx.try_recv().expect("BookingCreatedEvent was not published");
        assert_eq!(event.booking_id, 42);
        assert_eq!(event.itinerary_id, 4242);
        assert_eq!(event.rms_account_name, "DUMMY");
        assert_eq!(event.response, response);
    }

    #[tokio::test]
    async fn preflight_does_not_publish() {
        let state = AppState::new();
        let mut rx = state.event_bus.subscribe::<BookingCreatedEvent>();

        state.booking_service.book(true, request("DUMMY")).await.unwrap();

        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn failed_booking_does_not_publish() {
        let state = AppState::new();
        let mut rx = state.event_bus.subscribe::<BookingCreatedEvent>();

        assert!(state.booking_service.book(false, request("UNKNOWN")).await.is_err());

        assert!(rx.try_recv().is_err());
    }
}
