use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;
use super::guest::{DelegatingGuest, Guest, GuestInfo};
use super::pax_type::PaxType;

pub type GuestRef = Arc<dyn GuestInfo + Send + Sync>;

/// Real guests first, followed by delegating guests that fill each pax type up to its declared count.
#[derive(Clone)]
pub struct Guests {
    pub guests: Vec<GuestRef>,
    pub guest_numbers_map: HashMap<PaxType, u16>,
    by_pax_type: HashMap<PaxType, Vec<GuestRef>>,
}

impl Guests {
    pub fn new(real_guests: Vec<Guest>, guest_numbers_map: HashMap<PaxType, u16>) -> Self {
        debug_assert!(!real_guests.is_empty(), "Guests must have at least one guest");
        debug_assert!(
            !guest_numbers_map.is_empty(),
            "Guest numbers must not be empty"
        );

        let main = real_guests[0].clone();
        let mut actual: HashMap<&PaxType, usize> = HashMap::new();
        for g in &real_guests {
            *actual.entry(&g.pax_type).or_default() += 1;
        }

        let delegates: Vec<GuestRef> = guest_numbers_map
            .iter()
            .flat_map(|(pax_type, &count)| {
                let missing = (count as usize).saturating_sub(actual.get(pax_type).copied().unwrap_or(0));
                (0..missing).map(|_| {
                    Arc::new(DelegatingGuest::new(main.clone(), pax_type.clone())) as GuestRef
                })
            })
            .collect();

        let guests: Vec<GuestRef> = real_guests
            .into_iter()
            .map(|g| Arc::new(g) as GuestRef)
            .chain(delegates)
            .collect();

        let mut by_pax_type: HashMap<PaxType, Vec<GuestRef>> = HashMap::new();
        for g in &guests {
            by_pax_type.entry(g.pax_type().clone()).or_default().push(Arc::clone(g));
        }

        Guests { guests, guest_numbers_map, by_pax_type }
    }

    /// Guests of the given pax type (empty if there are none).
    pub fn by_pax_type(&self, pax_type: &PaxType) -> &[GuestRef] {
        self.by_pax_type.get(pax_type).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GuestRef> {
        self.guests.iter()
    }

    pub fn iter_by_pax_type<'a>(&'a self, pax_type: &'a PaxType) -> impl Iterator<Item = &'a GuestRef> {
        self.guests.iter().filter(move |g| g.pax_type() == pax_type)
    }

    pub fn total(&self) -> usize {
        self.guests.len()
    }

    pub fn main_guest(&self) -> &dyn GuestInfo {
        // real guests come first and there is always at least one
        self.guests[0].as_ref()
    }

    pub fn group_by_pax_type(&self) -> HashMap<PaxType, Vec<&GuestRef>> {
        let mut groups: HashMap<PaxType, Vec<&GuestRef>> = HashMap::new();
        for guest in &self.guests {
            groups.entry(guest.pax_type().clone()).or_default().push(guest);
        }
        groups
    }
}

impl<'a> IntoIterator for &'a Guests {
    type Item = &'a GuestRef;
    type IntoIter = std::slice::Iter<'a, GuestRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Index<&PaxType> for Guests {
    type Output = [GuestRef];

    /// `guests[&pax_type]` yields a slice; call `.iter()` for an iterator.
    fn index(&self, index: &PaxType) -> &Self::Output {
        self.by_pax_type(index)
    }
}

impl Index<PaxType> for Guests {
    type Output = [GuestRef];

    fn index(&self, index: PaxType) -> &Self::Output {
        self.by_pax_type(&index)
    }
}
