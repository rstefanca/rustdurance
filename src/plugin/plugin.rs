use std::fmt::{Debug, Formatter};

use async_trait::async_trait;

use crate::dto::booking_request::EnduranceVendorBooking;
use crate::dto::booking_response::BookingResponse;
use crate::error::PluginError;
use crate::plugin::plugin::PluginFeature::Booking;

pub enum PluginFeature {
    Booking(Box<dyn BookingPluginFeature>),
    Inventory(Box<dyn InventoryPluginFeature>)
}

#[async_trait]
pub trait BookingPluginFeature: Send + Sync {
    async fn book(
        &self,
        booking_request: EnduranceVendorBooking,
    ) -> Result<BookingResponse, PluginError>;
}

#[async_trait]
pub trait InventoryPluginFeature: Send + Sync {
    async fn get_inventory(&self) -> Result<(), PluginError>;
}

impl Debug for dyn InventoryPluginFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Inventory feature".to_string())
    }
}

pub struct EndurancePlugin  {
    pub rms: String,
    pub features: Vec<PluginFeature>
}

impl EndurancePlugin {
    pub fn new(
        rms: String,
        features: Vec<PluginFeature>,
    ) -> Self {
        EndurancePlugin {
            rms,
            features,
        }
    }

    pub async fn book(
        &self,
        booking_request: EnduranceVendorBooking,
    ) -> Result<BookingResponse, PluginError> {
       for f in &self.features {
           if let Booking(feature) = f {
               return feature.book(booking_request).await
           }
       }

        Err(PluginError::UnsupportedFeature("Booking".to_string()))
    }

    /*pub async fn get_inventory(&self) -> Result<(), PluginError> {
        for f in self.features {
            if let Some(booking_feature) = f.downcast_ref::<dyn InventoryPluginFeature>() {
                return booking_feature.get_inventory().await
            }
        }

        Err(PluginError::UnsupportedFeature("Booking"))
    }*/

}
