use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// ISO currency code, e.g. "USD".
// TODO: replace with an enum once the full `Currency` list is ported from headout-vendor-api.
pub type Currency = String;

/// Kotlin: `com.headout.vendor.api.models.Price`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub value: Decimal,
    pub currency: Currency,
}
