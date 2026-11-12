use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use super::common::{Currency, Price};

/// Kotlin: `com.headout.vendor.api.models.endurance.request.AMEnduranceBookingRequest`.
///
/// The class lives in the external `headout-vendor-api` jar, so the fields are
/// reconstructed from its constructor calls in Endurance's tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AMEnduranceBookingRequest {
    pub booking_id: i32,
    pub itinerary_id: i64,
    pub tour_id: i32,
    pub tour_currency: Currency,
    pub supply_partner_id: i32,
    pub supply_partner_name: String,
    pub rms_account_name: String,
    pub endurance_vendor_booking: EnduranceVendorBooking,
    #[serde(default)]
    pub reschedule_request: Option<bool>,
    #[serde(default)]
    pub success_callback_url: Option<String>,
    #[serde(default)]
    pub failure_callback_url: Option<String>,
    /// Kotlin: `FulfilmentProcess` enum (`DIRECT`, ...). Absent means `DIRECT`.
    // TODO: model as an enum once all variants are known.
    #[serde(default)]
    pub fulfilment_process: Option<String>,
    #[serde(default)]
    pub fulfilment_id: Option<i64>,
}

/// Kotlin: `com.headout.vendor.api.models.endurance.EnduranceVendorBooking`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnduranceVendorBooking {
    pub booking_id: i32,
    pub vendor_id: i64,
    pub tour_id: i32,
    pub inventory_date: NaiveDate,
    pub inventory_time: NaiveTime,
    pub product_name: String,
    /// Vendor-specific product code, itself a JSON document encoded as a string.
    pub product_code: String,
    pub primary_guest: VendorPrimaryGuest,
    /// Pax type (e.g. "ADULT") -> number of guests.
    pub guest_numbers_map: HashMap<String, i32>,
    #[serde(default)]
    pub guests: Option<Vec<VendorGuest>>,
    #[serde(default)]
    pub hotel_name: Option<String>,
    #[serde(default)]
    pub ticket_delivery_type: Option<String>,
    #[serde(default)]
    pub ticket_type: Option<String>,
    #[serde(default)]
    pub reserve_session_id: Option<String>,
    #[serde(default, rename = "thresholdPriceUSD")]
    pub threshold_price_usd: Option<Price>,
    #[serde(default)]
    pub threshold_price_local: Option<Price>,
    #[serde(default)]
    pub vendor_seats: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub vendor_bookables: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub vendor_bookable_codes: Option<Vec<String>>,
    #[serde(default)]
    pub reschedule_request: Option<bool>,
    #[serde(default)]
    pub success_callback_url: Option<String>,
    #[serde(default)]
    pub failure_callback_url: Option<String>,
    #[serde(default)]
    pub fulfilment_id: Option<i64>,
}

impl EnduranceVendorBooking {
    fn guest_count(&self) -> i32 {
        self.guest_numbers_map.values().sum()
    }
}

/// Kotlin: `com.headout.vendor.api.models.VendorPrimaryGuest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorPrimaryGuest {
    pub first_name: String,
    pub last_name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
}

/// Kotlin: `com.headout.vendor.api.models.VendorGuest`.
// TODO: fields are not visible in this repo; keep the raw JSON until the class is ported.
pub type VendorGuest = serde_json::Value;
