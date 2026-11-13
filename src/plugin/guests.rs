use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaxType {
    pub name: String,
}

impl From<&str> for PaxType {

    fn from(value: &str) -> Self {
       PaxType::from(value.to_string())
    }
}

impl From<String> for PaxType {
    fn from(value: String) -> Self {
        PaxType { name: value }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Guest {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) pax_type: PaxType,
}

#[derive(Clone)]
pub struct Guests {
    pub guests: Vec<Guest>,
    by_pax_type: HashMap<PaxType, Vec<Guest>>,
}

impl Guests {
    pub fn new(guests: Vec<Guest>) -> Self {
        if guests.is_empty() {
            panic!("Guests must have at least one guest");
        }
        let mut by_pax_type: HashMap<PaxType, Vec<Guest>> = HashMap::new();
        for guest in &guests {
            by_pax_type.entry(guest.pax_type.clone()).or_default().push(guest.clone());
        }
        Guests { guests, by_pax_type }
    }

    /// Guests of the given pax type (empty if there are none).
    pub fn by_pax_type(&self, pax_type: &PaxType) -> &[Guest] {
        self.by_pax_type.get(pax_type).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn iter_by_pax_type<'a>(&'a self, pax_type: &PaxType) -> impl Iterator<Item = &'a Guest> {
        self.by_pax_type(pax_type).iter()
    }
    pub fn total(&self) -> usize {
        self.guests.len()
    }

    pub fn main_guest(&self) -> &Guest {
        // there will always be at least one guest
        &self.guests.get(0).unwrap()
    }

    pub fn group_by_pax_type(&self) -> HashMap<PaxType, Vec<&Guest>> {
        let mut groups: HashMap<PaxType, Vec<&Guest>> = HashMap::new();
        for guest in &self.guests {
            groups.entry(guest.pax_type.clone()).or_default().push(guest);
        }
        groups
    }
}

impl IntoIterator for Guests {
    type Item = Guest;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.guests.into_iter()
    }
}

impl Index<&PaxType> for Guests {
    type Output = [Guest];

    /// `guests[&pax_type]` yields a slice; call `.iter()` for an iterator.
    fn index(&self, index: &PaxType) -> &Self::Output {
        self.by_pax_type(index)
    }
}

impl Index<PaxType> for Guests {
    type Output = [Guest];
    fn index(&self, index: PaxType) -> &Self::Output {
        self.by_pax_type.get(&index).unwrap()
    }
}
