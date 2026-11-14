use std::collections::HashMap;
use std::sync::Arc;

use crate::event_bus::EventBus;

use crate::plugin::dummy::dummy_plugin::dummy_plugin;
use crate::plugin::plugin::EndurancePlugin;
use crate::service::booking_service::{BookingService, PluginService};

/// Shared application state; the Rust counterpart of Spring's injected beans.
#[derive(Clone)]
pub struct AppState {
    pub booking_service: Arc<BookingService>,
    /// Application-wide event bus; subscribe here to react to domain events.
    pub event_bus: EventBus,
}

impl AppState {
    pub fn new() -> Self {
        let event_bus = EventBus::new();
        let plugins: Arc<Vec<Arc<EndurancePlugin>>> = Arc::new(vec![Arc::new(dummy_plugin(HashMap::new()))]);
        let plugin_service = PluginService::new(plugins);
        let booking_service = Arc::new(BookingService::new(plugin_service, event_bus.clone()));
        Self {
            booking_service,
            event_bus,
        }
    }
}
