use std::collections::HashMap;
use std::ops::Index;
use super::guest::Guest;
use super::pax_type::PaxType;

/// Real guests plus copies of the main guest (with the missing pax type) that fill each
/// pax type up to its declared count. `guests` is sorted by pax type so that
/// each pax type's guests form one contiguous slice.
#[derive(Clone)]
pub struct Guests {
    pub guests: Vec<Guest>,
    pub guest_numbers_map: HashMap<PaxType, u16>,
}

impl Guests {
    pub fn new(mut guests: Vec<Guest>, guest_numbers_map: HashMap<PaxType, u16>) -> Self {
        debug_assert!(!guests.is_empty(), "Guests must have at least one guest");
        debug_assert!(
            !guest_numbers_map.is_empty(),
            "Guest numbers must not be empty"
        );

        let main = guests[0].clone();

        let mut actual: HashMap<PaxType, usize> = HashMap::new();
        for g in &guests {
            *actual.entry(g.pax_type.clone()).or_default() += 1;
        }
        for (pax_type, &count) in &guest_numbers_map {
            let missing = (count as usize).saturating_sub(actual.get(pax_type).copied().unwrap_or(0));
            guests.extend((0..missing).map(|_| Guest { pax_type: pax_type.clone(), ..main.clone() }));
        }

        guests.sort_by(|a, b| a.pax_type.cmp(&b.pax_type)); // stable
        Guests { guests, guest_numbers_map }
    }

    /// Guests of the given pax type (empty if there are none).
    pub fn by_pax_type(&self, pax_type: &PaxType) -> &[Guest] {
        let start = self.guests.partition_point(|g| g.pax_type < *pax_type);
        let end = self.guests.partition_point(|g| g.pax_type <= *pax_type);
        &self.guests[start..end]
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Guest> {
        self.guests.iter()
    }

    pub fn iter_by_pax_type(&self, pax_type: &PaxType) -> impl Iterator<Item=&Guest> {
        self.by_pax_type(pax_type).iter()
    }

    pub fn total(&self) -> usize {
        self.guests.len()
    }

    pub fn main_guest(&self) -> &Guest {
        &self.guests[0]
    }

    pub fn group_by_pax_type(&self) -> HashMap<PaxType, Vec<&Guest>> {
        let mut groups: HashMap<PaxType, Vec<&Guest>> = HashMap::new();
        for guest in &self.guests {
            groups.entry(guest.pax_type.clone()).or_default().push(guest);
        }
        groups
    }
}

impl<'a> IntoIterator for &'a Guests {
    type Item = &'a Guest;
    type IntoIter = std::slice::Iter<'a, Guest>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        self.by_pax_type(&index)
    }
}
