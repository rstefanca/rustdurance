use super::pax_type::PaxType;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Guest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) pax_type: PaxType,
}

pub(super) struct DelegatingGuest {
    delegate: Guest,
    pax_type: PaxType,
}

impl DelegatingGuest {
    pub(super) fn new(delegate: Guest, pax_type: PaxType) -> Self {
        Self { delegate, pax_type }
    }
}

pub trait GuestInfo {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
    fn pax_type(&self) -> &PaxType;
}

impl GuestInfo for Guest {
    fn first_name(&self) -> &str { &self.first_name }
    fn last_name(&self) -> &str { &self.last_name }
    fn pax_type(&self) -> &PaxType { &self.pax_type }
}

impl GuestInfo for DelegatingGuest {
    fn first_name(&self) -> &str { &self.delegate.first_name }
    fn last_name(&self) -> &str { &self.delegate.last_name }
    // overrides the delegate's pax_type
    fn pax_type(&self) -> &PaxType { &self.pax_type }
}
