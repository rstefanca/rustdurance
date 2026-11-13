use crate::dto::booking_request::{EnduranceVendorBooking, VendorGuest};
use crate::error::PluginError;
use crate::plugin::guest::{Guest, GuestInfo};
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
use std::sync::Arc;

/// Port of Endurance's `BookingService`. Only the entry point exists so far.
pub struct BookingService {
    plugin_service: PluginService,
}

impl BookingService {
    pub fn new(plugin_service: PluginService) -> Self {
        Self { plugin_service }
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

        let br = &BookingRequestImpl::new(request.endurance_vendor_booking);

        let response = plugin.book(br).await.map_err(lift_error)?;

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
            })
            .collect::<Vec<Guest>>();


       let pax_numbers = delegate.guest_numbers_map.iter()
           .map(|(k,v)| (PaxType::from(k.as_str()), *v))
           .collect();

        let guests = Guests::new(list.clone(), pax_numbers);
        Self { delegate, guests }
    }

    //delegate
    fn main_guest(&self) -> &dyn GuestInfo {
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
