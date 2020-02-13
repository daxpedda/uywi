//! UYWI concepts.

use crate::*;
use array_init::array_init;
use arrayvec::ArrayVec;
use std::mem;

/// A concept.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Concept {
	/// Concepts are stored as [`Radical`]s.
	radicals: [Radical; 4],
	/// Concept length.
	length: Length,
}

impl Concept {
	/// Build [`Concept`] from members. Used for internal purposes.
	pub(crate) const fn new(radicals: [Radical; 4], length: Length) -> Self {
		return Self { radicals, length };
	}

	/// Build [`Concept`] from concept index.
	/// # Errors
	/// [`Error::ConceptIndexInvalid`] if `index` is higher then the number of pages of the given [`Length`]
	pub fn from_index(index: usize, length: Length) -> Result<Self> {
		// check for valid concept index
		if index >= length.num_of_concepts() {
			return Err(Error::ConceptIndexInvalid);
		}

		// storing the radical indexes
		let mut radicals = ArrayVec::<[_; 4]>::new();
		// the concept index we start at
		let mut index_left = index;

		// get interval at which each position's radical changes
		for (position, interval) in length.radical_intervals().into_iter().enumerate() {
			// how many intervals fit into the left concept index
			let intervals = index_left.pdiv(interval);

			// get radical indexes with the appropriate ordering
			let mut radicals_ordered = length.radicals_ordered(position, &radicals);

			// get the radical
			let radical = radicals_ordered.nth(intervals).expect("no radical found at given interval");

			// drop the iterator because we need a `&mut` to `radicals`
			mem::drop(radicals_ordered);

			// store it into the concept
			radicals.push(radical);

			// store the concept index thats left
			index_left = index_left.psub(intervals.pmul(interval));
		}

		// cache order
		let radical_order = length.radical_order();

		// reorder them
		let radicals = array_init(|index| {
			// check if there is a radical
			return if let Some(radical) = radical_order.get(index) {
				radicals[usize::from(*radical)]
			}
			// fill it with `0`s if not
			else {
				Radical::from_index(0)
			};
		});

		return Ok(Self { radicals, length });
	}

	/// Build [`Concept`] from index string.
	/// # Errors
	/// [`Error::ConceptStringInvalid`] if `index` can not be parsed into `usize`
	///
	/// [`Error::ConceptStringNull`] if `index` is 0
	///
	/// [`Error::ConceptStringInvalid`] if `index` is higher then the number of pages of the given [`Length`]
	pub fn from_index_str(index: &str, length: Length) -> Result<Self> {
		let index: usize = if let Ok(index) = index.parse() {
			index
		} else {
			return Err(Error::ConceptStringInvalid);
		};

		// in index form a concept is always `- 1` to the string
		return Self::from_index(index.checked_sub(1).ok_or(Error::ConceptStringNull)?, length);
	}

	/// Get radicals.
	pub(crate) fn radicals(self) -> ArrayVec<[Radical; 4]> {
		let mut radicals = ArrayVec::new();

		for index in 0..self.length.as_int() {
			radicals.push(self.radicals[index]);
		}

		return radicals;
	}

	/// Get index.
	#[must_use]
	pub fn index(self) -> usize {
		// store calculated concept index
		let mut concept_index = 0;
		// store already used radicals
		let mut radicals_used = ArrayVec::<[_; 4]>::new();
		// cache radical intervals
		let radical_intervals = self.length.radical_intervals();

		for (position, order) in self.length.radical_order_mirrored().iter().enumerate() {
			// get radical from correct position
			let radical = self.radicals()[usize::from(*order)];
			// get radicals in appropriate order for this position and filter already used radicals
			let mut radicals = self.length.radicals_ordered(position, &radicals_used);

			// find the ordered radical index
			let ordered_radical_index = radicals
				.position(|ordered_radical| return ordered_radical == radical)
				.expect("radical not found");

			// add index multiplied with interval size to the concept index
			concept_index = concept_index.padd(ordered_radical_index.pmul(radical_intervals[position]));

			// drop the iterator because we need a `&mut` to `radicals_used`
			mem::drop(radicals);

			// store found radical
			radicals_used.push(radical);
		}

		return concept_index;
	}

	/// Get index as string.
	#[must_use]
	pub fn index_as_string(self) -> String {
		// in string form a concept is always `+ 1` to the index
		return self.index().padd(1).to_string();
	}

	/// Get concept as [`String`].
	#[must_use]
	pub fn to_string(self, script: Script) -> String {
		return script.concept(self).to_string();
	}

	/// Get length.
	#[must_use]
	pub const fn length(self) -> Length {
		return self.length;
	}

	/// Get [`Page`] that concept is in.
	#[must_use]
	pub fn page(self) -> Page {
		return Page::from_index(self.index().pdiv(self.length.concepts_per_page()), self.length).expect("couldn't calculate page from concept");
	}
}

impl IntoIterator for Concept {
	type Item = Stem;
	type IntoIter = Stems;

	#[must_use]
	fn into_iter(self) -> Self::IntoIter {
		return Stems::new(self);
	}
}

/// Convenience type to make it easier to deal with radicals.
/// We don't really want to jumble around with [`str`]s and we don't want to accidentally do calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Radical {
	/// Index of the radical. Corresponds to [`ScriptExtPriv::radicals`].
	index: u8,
}

impl Radical {
	/// Build new [`Radical`] from index.
	pub fn from_index(index: u8) -> Self {
		assert!(usize::from(index) < NUM_OF_RADICALS, "invalid radical");

		return Self { index };
	}

	/// Get index of radical.
	pub const fn index(self) -> u8 {
		return self.index;
	}
}
