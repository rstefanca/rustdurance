//! A booking's party of guests, grouped and ordered by [`PaxType`].
//!
//! [`Guests`] holds the real guests of a booking plus synthetic "filler" guests so that
//! every pax type has as many entries as its declared count. Because the list is kept
//! sorted by pax type, each type occupies one contiguous slice and lookups by type are
//! cheap binary searches.

use super::guest::Guest;
use super::pax_type::PaxType;
use std::collections::HashMap;
use std::ops::Index;

// region: --- Types

/// Real guests plus copies of the main guest (with the missing pax type) that fill each
/// pax type up to its declared count. `guests` is sorted by pax type so that
/// each pax type's guests form one contiguous slice.
///
/// Construct it with [`Guests::new`]; building the struct by hand skips the padding and
/// sorting, and [`Guests::by_pax_type`] relies on the sort order.
#[derive(Clone)]
pub struct Guests {
    /// All guests, real and filler, sorted by pax type (stable, so guests of the same
    /// type keep their input order).
    pub guests: Vec<Guest>,
    /// Declared number of guests per pax type, with zero counts removed.
    pub guest_numbers_map: HashMap<PaxType, u16>,
}

// endregion: --- Types

impl Guests {
    /// Builds a [`Guests`] from the real `guests` and the declared per-type counts.
    ///
    /// - The first element of `guests` is treated as the main guest.
    /// - Entries in `guest_numbers_map` with a count of `0` are dropped.
    /// - For every pax type whose declared count exceeds the number of real guests of
    ///   that type, the shortfall is filled with clones of the main guest whose
    ///   `pax_type` is replaced by the missing type.
    /// - Real guests are never removed: if a type has more real guests than declared, or
    ///   a type is absent from the map, they are kept as they are.
    /// - The result is sorted (stably) by pax type.
    ///
    /// Note that sorting happens after padding, so [`Guests::main_guest`] returns the
    /// first guest of the lowest pax type, which is not necessarily the guest passed
    /// first to this function.
    ///
    /// # Panics
    ///
    /// In debug builds, if `guests` or `guest_numbers_map` is empty, or if there are more
    /// real guests than the declared counts add up to. In release builds an empty
    /// `guests` panics later, on the `guests[0]` access.
    ///
    /// `guests` is borrowed and cloned; the caller's vector is left untouched.
    pub fn new(guests: &Vec<Guest>, guest_numbers_map: HashMap<PaxType, u16>) -> Self {
        let total_e_guests = guest_numbers_map
            .iter()
            .fold(0, |acc, (_, v)| acc + v.clone());
        let guest_size = guests.len() as u16;

        debug_assert!(!guests.is_empty(), "Guests must have at least one guest");
        debug_assert!(
            !guest_numbers_map.is_empty(),
            "Guest numbers must not be empty"
        );
        debug_assert!(
            guest_size <= total_e_guests,
            "Too many guest guests. Expected {}, got {}",
            total_e_guests,
            guest_size
        );

        let cleaned_numbers: HashMap<PaxType, u16> = guest_numbers_map
            .into_iter()
            .filter(|&(_, value)| value > 0)
            .collect();

        let mut guests = guests.clone();
        let main = guests[0].clone();

        let mut actual: HashMap<PaxType, usize> = HashMap::new();
        for g in &guests {
            *actual.entry(g.pax_type.clone()).or_default() += 1;
        }
        for (pax_type, &count) in &cleaned_numbers {
            let missing =
                (count as usize).saturating_sub(actual.get(pax_type).copied().unwrap_or(0));
            guests.extend((0..missing).map(|_| Guest {
                pax_type: pax_type.clone(),
                ..main.clone()
            }));
        }

        guests.sort_by(|a, b| a.pax_type.cmp(&b.pax_type)); // stable
        Guests {
            guests,
            guest_numbers_map: cleaned_numbers,
        }
    }

    /// Returns the guests of the given pax type as a contiguous slice, or an empty slice
    /// if there are none. Runs in `O(log n)` thanks to the sort invariant.
    pub fn by_pax_type(&self, pax_type: &PaxType) -> &[Guest] {
        let start = self.guests.partition_point(|g| g.pax_type < *pax_type);
        let end = self.guests.partition_point(|g| g.pax_type <= *pax_type);
        &self.guests[start..end]
    }

    /// Iterates over all guests in pax type order.
    pub fn iter(&self) -> std::slice::Iter<'_, Guest> {
        self.guests.iter()
    }

    /// Iterates over the guests of the given pax type (empty if there are none).
    pub fn iter_by_pax_type(&self, pax_type: &PaxType) -> impl Iterator<Item = &Guest> {
        self.by_pax_type(pax_type).iter()
    }

    /// Total number of guests, including filler guests.
    pub fn total(&self) -> usize {
        self.guests.len()
    }

    /// The first guest in sorted order.
    ///
    /// This is the first guest of the lowest pax type, which may differ from the guest
    /// that was first in the input of [`Guests::new`] (the one used as the template for
    /// filler guests).
    pub fn main_guest(&self) -> &Guest {
        &self.guests[0]
    }

    /// Groups the guests by pax type. Within each group, guests keep their sorted order.
    /// Unlike [`Guests::by_pax_type`], this allocates and the map has no defined
    /// iteration order.
    pub fn group_by_pax_type(&self) -> HashMap<PaxType, Vec<&Guest>> {
        let mut groups: HashMap<PaxType, Vec<&Guest>> = HashMap::new();
        for guest in &self.guests {
            groups
                .entry(guest.pax_type.clone())
                .or_default()
                .push(guest);
        }
        groups
    }
}

// region: --- Iterator Implementations

impl<'a> IntoIterator for &'a Guests {
    type Item = &'a Guest;
    type IntoIter = std::slice::Iter<'a, Guest>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// endregion: --- Iterator Implementations

// region: --- Support

impl Index<&PaxType> for Guests {
    type Output = [Guest];

    /// `guests[&pax_type]` yields a slice; call `.iter()` for an iterator.
    /// Equivalent to [`Guests::by_pax_type`]; empty if the type has no guests.
    fn index(&self, index: &PaxType) -> &Self::Output {
        self.by_pax_type(index)
    }
}

impl Index<PaxType> for Guests {
    type Output = [Guest];

    /// Same as indexing by `&PaxType`, taking the pax type by value.
    fn index(&self, index: PaxType) -> &Self::Output {
        self.by_pax_type(&index)
    }
}

// endregion: --- Support
