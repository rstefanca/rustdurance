use std::any::Any;
use std::collections::HashMap;

use async_trait::async_trait;

use crate::dto::booking_response::{BookingResponse, Ticket};
use crate::dto::utils::booking_response;
use crate::error::PluginError;
use crate::plugin::guests::{Guest, PaxType};
use crate::plugin::plugin::{BookingPluginFeature, BookingRequest, EndurancePlugin, PluginFeature};


pub struct DummyPlugin {
    api: DummyApi,
}

pub struct DummyApi {
    
}

impl DummyApi {
    
    pub async fn get_products(&self) -> Vec<String> {
        vec!["Product".to_string()]
    }
    
}

impl DummyPlugin {
    pub fn new() -> Self {
        Self {
            api: DummyApi {}
        }
    }
}

struct DummyBooking {
    plugin: DummyPlugin
}

impl DummyBooking {
    fn new(plugin: DummyPlugin) -> Self {
        DummyBooking {
            plugin
        }
    }
}

#[async_trait]
impl BookingPluginFeature for  DummyBooking  {
    async fn book(
        &self,
        booking_request: &dyn BookingRequest,
    ) -> Result<BookingResponse, PluginError> {
        let _ = self.plugin.api.get_products();
        let guests = booking_request.guests();
        let _adults = guests[PaxType::from("ADULT")].iter();
        let tickets: Vec<Ticket> = Vec::new();
        Ok(booking_response("1", tickets))
    }
}

pub fn dummy_plugin(properties: HashMap<String, Box<dyn Any>>) -> EndurancePlugin {
    let plugin = DummyPlugin::new();
    let booking_feature = PluginFeature::Booking(Box::new(DummyBooking::new(plugin)));
   
    EndurancePlugin::new("DUMMY".to_string(),  vec![booking_feature])
}
