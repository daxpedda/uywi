//! Length configuration.

use crate::{structure, *};
use arrayvec::ArrayVec;
use std::fmt::{self, Display, Formatter};

/// Concept length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
	/// Two radicals.
	L2,
	/// Three radicals.
	L3,
	/// Four radicals.
	L4,
}

#[allow(clippy::unused_self)]
impl Length {
	/// Build new [`Length`].
	/// # Errors
	/// [`Error::LengthInvalid`] if `length` has no corresponding [`Length`]
	pub fn new(length: usize) -> Result<Self> {
		match length {
			2 => return Ok(Self::L2),
			3 => return Ok(Self::L3),
			4 => return Ok(Self::L4),
			_ => return Err(Error::LengthInvalid),
		}
	}

	/// Get [`Length`] as number.
	#[must_use]
	pub fn as_int(self) -> usize {
		return match self {
			Self::L2 => 2,
			Self::L3 => 3,
			Self::L4 => 4,
		};
	}

	/// Total number of pages.
	#[must_use]
	pub fn num_of_pages(self) -> usize {
		return match self {
			Self::L2 => 1,
			Self::L3 => NUM_OF_RADICALS,
			// `Radicals * (Radicals - 1)`
			Self::L4 => NUM_OF_RADICALS.pmul(NUM_OF_RADICALS.psub(1)),
		};
	}

	/// Number of concepts in a page.
	#[must_use]
	pub fn concepts_per_page(self) -> usize {
		// `RowsPerPage * ConceptsPerRow`
		return self.rows_per_page().pmul(self.concepts_per_row());
	}

	/// Number of rows in a page.
	#[must_use]
	pub fn rows_per_page(self) -> usize {
		return match self {
			Self::L2 => NUM_OF_RADICALS,
			// `Radicals - 1`
			Self::L3 => NUM_OF_RADICALS.psub(1),
			// `Radicals - 2`
			Self::L4 => NUM_OF_RADICALS.psub(2),
		};
	}

	/// Number of concepts in a row.
	#[must_use]
	pub fn concepts_per_row(self) -> usize {
		return match self {
			// `Radicals - 1`
			Self::L2 => NUM_OF_RADICALS.psub(1),
			// `Radicals - 2`
			Self::L3 => NUM_OF_RADICALS.psub(2),
			// `Radicals - 3`
			Self::L4 => NUM_OF_RADICALS.psub(3),
		};
	}

	/// Total number of concepts.
	#[must_use]
	pub fn num_of_concepts(self) -> usize {
		// `ConceptsPerPage * NumberOfPages`
		return self.num_of_pages().pmul(self.concepts_per_page());
	}

	/// Number of stems in a concept.
	#[must_use]
	pub fn stems_per_concept(self) -> usize {
		return structure::list(self).len();
	}

	/// Number of words in a stem.
	#[must_use]
	pub fn words_per_stem(self) -> usize {
		return self.num_of_forms();
	}

	/// Number of forms.
	#[must_use]
	pub fn num_of_forms(self) -> usize {
		return match self {
			Self::L2 => 2,
			Self::L3 | Self::L4 => 4,
		};
	}

	/// Interval at which each radical position changes when iterated.
	pub(crate) fn radical_intervals(self) -> ArrayVec<[usize; 4]> {
		let mut intervals = ArrayVec::new();

		match self {
			Self::L2 => intervals.try_extend_from_slice(&[NUM_OF_RADICALS.psub(1), 1]),
			Self::L3 => intervals.try_extend_from_slice(&[(NUM_OF_RADICALS.psub(2)).pmul(NUM_OF_RADICALS.psub(1)), NUM_OF_RADICALS.psub(2), 1]),
			Self::L4 => intervals.try_extend_from_slice(&[
				(NUM_OF_RADICALS.psub(3)).pmul(NUM_OF_RADICALS.psub(2)).pmul(NUM_OF_RADICALS.psub(1)),
				(NUM_OF_RADICALS.psub(3)).pmul(NUM_OF_RADICALS.psub(2)),
				NUM_OF_RADICALS.psub(3),
				1,
			]),
		}
		.expect("failed to fill stem config internals");

		return intervals;
	}

	/// List of positions radicals are ordered in inside a concept.
	pub(crate) fn radical_order(self) -> ArrayVec<[u8; 4]> {
		let mut order = ArrayVec::new();

		match self {
			Self::L2 => order.try_extend_from_slice(&[1, 0]),
			Self::L3 => order.try_extend_from_slice(&[1, 2, 0]),
			Self::L4 => order.try_extend_from_slice(&[1, 3, 2, 0]),
		}
		.expect("failed to fill stem config internals");

		return order;
	}

	/// On some occasions we need the [`Length::radical_order`] mirrored to reflect reverse usage.
	pub(crate) fn radical_order_mirrored(self) -> ArrayVec<[u8; 4]> {
		let mut order = ArrayVec::new();
		let radical_order = self.radical_order();

		for position in 0..self.as_int() {
			order.push(
				radical_order
					.iter()
					.position(|order| {
						return usize::from(*order) == position;
					})
					.expect("no order found at that position")
					.pinto(),
			);
		}

		return order;
	}

	/// Where radicals start on each position in the concept.
	fn radical_start(self) -> ArrayVec<[u8; 4]> {
		let mut order = ArrayVec::new();

		match self {
			Self::L2 => order.try_extend_from_slice(&[3, 0]),
			Self::L3 => order.try_extend_from_slice(&[3, 0, 1]),
			Self::L4 => order.try_extend_from_slice(&[3, 0, 2, 1]),
		}
		.expect("failed to fill stem config internals");

		return order;
	}

	/// Iterator over [`Radical`]s with the correct ordering depending on position in concept.
	/// Filter [`Radical`]s through `filter`.
	pub(crate) fn radicals_ordered<'a>(self, position: usize, filter: &'a [Radical]) -> impl 'a + Iterator<Item = Radical> {
		// by accident the positions at where radicals start are also the mirrored positions of radicals in a concept
		let start = self.radical_start()[position];

		// create ordered iterator
		return (start..NUM_OF_RADICALS.pinto()).chain(0..start).filter_map(move |value| {
			// remove all used radicals
			for radical in filter.iter() {
				if value == radical.index() {
					return None;
				}
			}

			// map radical indexes to `Radical`
			return Some(Radical::from_index(value));
		});
	}
}

impl Default for Length {
	#[must_use]
	fn default() -> Self {
		return Self::L4;
	}
}

impl Display for Length {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		return write!(
			formatter,
			"{}",
			match self {
				Self::L2 => 2,
				Self::L3 => 3,
				Self::L4 => 4,
			}
		);
	}
}
