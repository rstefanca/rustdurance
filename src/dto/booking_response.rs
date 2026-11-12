use serde::{Deserialize, Serialize};

use super::common::Price;

/// Kotlin: `com.headout.vendor.api.models.BookingResponse`.
///
/// Returned as-is, without the `ApiResp` wrapper used by Endurance.
/// Fields are reconstructed from `BookingPostProcessingService`, which copies every one of them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingResponse {
    #[serde(default)]
    pub order_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_id: Option<String>,
    #[serde(default)]
    pub tickets: Vec<Ticket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_amount: Option<Price>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_hotel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_time: Option<String>,
    /// Kotlin: `TitledAdditionalInfo` (a title plus a list of lines).
    // TODO: type it once the class is ported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voucher_display_info: Option<serde_json::Value>,
}

/// Kotlin: `com.headout.vendor.api.models.Ticket`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    #[serde(rename = "type")]
    pub ticket_type: TicketType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_id: Option<String>,
}

/// Variants seen in Endurance plugins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketType {
    Qrcode,
    QrcodeUrl,
    Barcode,
    BarcodeUrl,
    PdfUrl,
    HtmlUrl,
    AppleWalletUrl,
    Text,
}

