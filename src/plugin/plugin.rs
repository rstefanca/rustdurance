use std::fmt::{Debug, Formatter};

use crate::dto::booking_response::BookingResponse;
use crate::error::PluginError;
use crate::plugin::plugin::PluginFeature::{Booking, Inventory};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::plugin::guests::{Guest, Guests};

pub enum PluginFeature {
    Booking(Box<dyn BookingPluginFeature>),
    Inventory(Box<dyn InventoryPluginFeature>),
}

impl PluginFeature {
    fn kind(&self) -> FeatureKind {
        match &self {
            Booking(_) => FeatureKind::Booking,
            Inventory(_) => FeatureKind::Inventory,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum FeatureKind {
    Booking,
    Inventory,
}

#[async_trait]
pub trait BookingPluginFeature: Send + Sync {
    async fn book(
        &self,
        booking_request: &dyn BookingRequest,
    ) -> Result<BookingResponse, PluginError>;
}

#[async_trait]
pub trait InventoryPluginFeature: Send + Sync {
    async fn get_inventory(&self) -> Result<(), PluginError>;
}


pub struct EndurancePlugin {
    pub rms_account_name: String, // multiple account per RMS
    pub features: Vec<PluginFeature>,
}

impl Debug for EndurancePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rms: {}, features: {:?}", self.rms_account_name, self.list_features())
    }
}

pub trait BookingRequest: Send + Sync {
    fn booking_id(&self) -> &u32;

    fn product_code(&self) -> &String;
    
    fn booking_date(&self) -> &NaiveDate;
    
    fn booking_time(&self) -> &NaiveTime;
    
    fn booking_date_time(&self) -> NaiveDateTime;
    
    fn guests(&self) -> &Guests;
}

impl EndurancePlugin {
    pub fn new(rms: String, features: Vec<PluginFeature>) -> Self {
        EndurancePlugin { rms_account_name: rms, features }
    }

    pub async fn book(
        &self,
        booking_request: &dyn BookingRequest,
    ) -> Result<BookingResponse, PluginError> {
        for feature in &self.features {
            if let Booking(booking_feature) = feature {
                return booking_feature.book(booking_request).await;
            }
        }

        Err(self.unsupported(FeatureKind::Booking))
    }

    pub async fn get_inventory(&self) -> Result<(), PluginError> {
        for feature in &self.features {
            if let Inventory(inventory_feature) = feature {
                return inventory_feature.get_inventory().await;
            }
        }

        Err(self.unsupported(FeatureKind::Inventory))
    }

    fn unsupported(&self, feature_kind: FeatureKind) -> PluginError {
        PluginError::UnsupportedFeature(feature_kind, self.list_features())
    }

    /**
     * Checks if a plugin has given feature.
     **/
    pub fn has_feature(&self, feature_kind: &FeatureKind) -> bool {
        self.features.iter().any(|f| f.kind() == *feature_kind)
    }

    pub fn list_features(&self) -> Vec<FeatureKind> {
        self.features.iter().map(|f| f.kind()).collect()
    }
}
