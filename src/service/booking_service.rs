use std::sync::Arc;
use crate::{dto::{booking_request::AMEnduranceBookingRequest, booking_response::BookingResponse}, error::AppError};
use crate::error::PluginError;
use crate::plugin::plugin::{EndurancePlugin};
pub(crate) use crate::service::plugin_service::PluginService;

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
        _preflight: bool,
        request: AMEnduranceBookingRequest,
    ) -> Result<BookingResponse, AppError> {
        let plugin = self.resolve_plugin(&request.rms_account_name)?;

        plugin.book(request.endurance_vendor_booking)
            .await.map_err(lift_error)
    }

    fn resolve_plugin(&self, rms: &str) -> Result<Arc<EndurancePlugin>, AppError> {
        self.plugin_service.resolve_plugin(rms)
    }
}

fn lift_error(plugin_error: PluginError) -> AppError {
    AppError::PluginError { cause: plugin_error }
}
