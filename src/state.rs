use std::collections::HashMap;
use std::sync::Arc;

use crate::plugin::dummy::dummy_plugin::{dummy_plugin, DummyPlugin};
use crate::plugin::plugin::{EndurancePlugin};
use crate::service::booking_service::{PluginService, BookingService};

/// Shared application state; the Rust counterpart of Spring's injected beans.
#[derive(Clone)]
pub struct AppState {
    pub booking_service: Arc<BookingService>,
    
}

impl AppState {
    pub fn new() -> Self {
        let plugins: Arc<Vec<Arc<EndurancePlugin>>> = Arc::new(vec![Arc::new(dummy_plugin(HashMap::new()))]);
        let plugin_service = PluginService::new(plugins);
        let booking_service = Arc::new(BookingService::new(plugin_service));
        Self {
            booking_service,
        }
    }
}
