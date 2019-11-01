//! Configuration for all word types.

use super::{Error, Result};
use crate::util::*;
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
			Self::L3 => Self::RADICALS.len(),
			// `Radicals * (Radicals - 1)`
			Self::L4 => Self::RADICALS.len().pmul(Self::RADICALS.len().psub(1)),
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
			Self::L2 => Self::RADICALS.len(),
			// `Radicals - 1`
			Self::L3 => Self::RADICALS.len().psub(1),
			// `Radicals - 2`
			Self::L4 => Self::RADICALS.len().psub(2),
		};
	}

	/// Number of concepts in a row.
	#[must_use]
	pub fn concepts_per_row(self) -> usize {
		return match self {
			// `Radicals - 1`
			Self::L2 => Self::RADICALS.len().psub(1),
			// `Radicals - 2`
			Self::L3 => Self::RADICALS.len().psub(2),
			// `Radicals - 3`
			Self::L4 => Self::RADICALS.len().psub(3),
		};
	}

	/// Total number of concepts.
	#[must_use]
	pub fn num_of_concepts(self) -> usize {
		// `ConceptsPerPage * NumberOfPages`
		return self.num_of_pages().pmul(self.concepts_per_page());
	}

	/// List of how stems are configured.
	fn stem_configs_internal(self) -> ArrayVec<[&'static str; 8]> {
		let mut configs = ArrayVec::new();

		match self {
			Self::L2 => configs.try_extend_from_slice(&["r0 v0 r1", "r0 v0 v0 r1", "r0 v0 r1 r1 v0"]),
			Self::L3 => configs.try_extend_from_slice(&[
				"r0 v0 r1 v1 r2",
				"r0 v0 r1 r1 v1 r2",
				"r0 v0 v0 r1 v1 r2",
				"r0 v0 r1 v1 v1 r2",
				"r0 v0 r1 v1 r2 r2 v1",
			]),
			Self::L4 => configs.try_extend_from_slice(&[
				"r0 v0 r1 r2 v1 r3",
				"r0 v0 r1 r1 r2 v1 r3",
				"r0 v0 v0 r1 r2 v1 r3",
				"r0 v0 r1 r2 v1 v1 r3",
				"r0 v0 r1 r2 v1 r3 r3 v1",
				"r0 v0 r1 v0 v0 r2 v1 r3",
				"r0 v0 r1 v0 r2 r2 v1 r3",
				"r0 v0 r1 v0 r2 v1 v1 r3",
			]),
		}
		.expect("failed to fill stem config internals");

		return configs;
	}

	/// Number of stems in a concept.
	#[must_use]
	pub fn stems_per_concept(self) -> usize {
		return self.stem_configs_internal().len();
	}

	/// Number of forms in a stem.
	#[must_use]
	pub fn forms_per_stem(self) -> usize {
		return self.forms_config().len();
	}

	/// Stem configuration list.
	pub(super) fn stem_configs(self) -> ArrayVec<[ArrayVec<[Letter; 8]>; 8]> {
		let mut configs = ArrayVec::new();
		let configs_internal = self.stem_configs_internal();

		for index in 0..self.stems_per_concept() {
			let mut config = ArrayVec::new();

			for letter in configs_internal[index].split_whitespace() {
				debug_assert!(letter.len() == 2, "configuration option has not exactly 2 letters");

				let r#type = &letter[0..1];
				let index: u8 = letter[1..2].parse().expect("configuration index isn't a number");

				match r#type {
					// radical
					"r" => config.push(Letter::new_radical(self, index)),
					// vocal
					"v" => config.push(Letter::new_vocal(usize::from(index))),
					_ => unreachable!("configuration type not valid"),
				}
			}

			configs.push(config);
		}

		return configs;
	}

	/// List how forms are configured.
	pub(super) fn forms_config(self) -> ArrayVec<[Vocals; 4]> {
		let mut configs = ArrayVec::new();

		match self {
			Self::L2 => configs.try_extend_from_slice(&[Vocals("o", "o"), Vocals("ı", "ı")]),
			Self::L3 | Self::L4 => configs.try_extend_from_slice(&[Vocals("o", "o"), Vocals("o", "ı"), Vocals("ı", "o"), Vocals("ı", "ı")]),
		}
		.expect("failed to fill stem config internals");

		return configs;
	}

	/// List of radicals.
	const RADICALS: [&'static str; 44] = [
		"?", "Y", "w", "h", "2", "H", "K", "k", "X", "x", "8", "4", "G", "g", "j", "7", "3", "Q", "c", "9", "S", "s", "Z", "z", "D", "d", "T", "t",
		"P", "0", "B", "6", "V", "f", "p", "b", "m", "n", "O", "R", "r", "1", "L", "l",
	];

	/// Interval at which each radical position changes when iterated.
	pub(super) fn radical_intervals(self) -> ArrayVec<[usize; 4]> {
		let mut intervals = ArrayVec::new();

		match self {
			Self::L2 => intervals.try_extend_from_slice(&[Self::RADICALS.len().psub(1), 1]),
			Self::L3 => intervals.try_extend_from_slice(&[
				(Self::RADICALS.len().psub(2)).pmul(Self::RADICALS.len().psub(1)),
				Self::RADICALS.len().psub(2),
				1,
			]),
			Self::L4 => intervals.try_extend_from_slice(&[
				(Self::RADICALS.len().psub(3))
					.pmul(Self::RADICALS.len().psub(2))
					.pmul(Self::RADICALS.len().psub(1)),
				(Self::RADICALS.len().psub(3)).pmul(Self::RADICALS.len().psub(2)),
				Self::RADICALS.len().psub(3),
				1,
			]),
		}
		.expect("failed to fill stem config internals");

		return intervals;
	}

	/// List of positions radicals are ordered in inside a concept.
	pub(super) fn radical_order(self) -> ArrayVec<[u8; 4]> {
		let mut order = ArrayVec::new();

		match self {
			Self::L2 => order.try_extend_from_slice(&[1, 0]),
			Self::L3 => order.try_extend_from_slice(&[1, 2, 0]),
			Self::L4 => order.try_extend_from_slice(&[1, 3, 2, 0]),
		}
		.expect("failed to fill stem config internals");

		return order;
	}

	/// On some occasions we need the [`RADICAL_ORDER`] mirrored to reflect reverse usage.
	pub(super) fn radical_order_mirrored(self) -> ArrayVec<[u8; 4]> {
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
	pub(super) fn radicals_ordered<'a>(self, position: usize, filter: &'a [Radical]) -> impl 'a + Iterator<Item = Radical> {
		// by accident the positions at where radicals start are also the mirrored positions of radicals in a concept
		let start = self.radical_start()[position];

		// create ordered iterator
		return (start..Self::RADICALS.len().pinto()).chain(0..start).filter_map(move |value| {
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

/// Represents a radical or a vocal from [`FORMS_CONFIG`].
#[derive(Clone, Copy, Debug)]
pub(super) enum Letter {
	/// Represents a radical position.
	Radical(u8),
	/// Represents a vocal from [`FORMS_CONFIG`].
	Vocal(Vocal),
}

impl Letter {
	/// Build new [`Radical`](Letter::Radical).
	fn new_radical(length: Length, index: u8) -> Self {
		assert!(usize::from(index) < length.as_int(), "index bigger than the size of a concept");

		return Self::Radical(index);
	}

	/// Build new [`Vocal`](Letter::Vocal).
	fn new_vocal(index: usize) -> Self {
		return Self::Vocal(match index {
			0 => Vocal::First,
			1 => Vocal::Last,
			_ => unreachable!("configuration type `v` has an index bigger than there are available vocals"),
		});
	}
}

/// Convenience type to make it easier to deal with radicals.
/// We don't really want to jumble around with [`str`] and we don't want to accidentally do calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Radical {
	/// Index of the radical. Corresponds to [`RADICALS`].
	index: u8,
}

impl Radical {
	/// Build new [`Radical`] from index.
	pub fn from_index(index: u8) -> Self {
		assert!(usize::from(index) < Length::RADICALS.len(), "invalid radical");

		return Self { index };
	}

	/// Build new [`Radical`] from string.
	pub fn from_str(string: &str) -> Result<Self> {
		return if let Some(radical) = Length::RADICALS.iter().position(|radical| return radical == &string) {
			Ok(Self::from_index(radical.pinto()))
		} else {
			Err(Error::ConceptRadicalInvalid)
		};
	}

	/// Get [`str`] represantation of this radical.
	pub fn as_str(self) -> &'static str {
		return Length::RADICALS[usize::from(self.index)];
	}

	/// Get index of radical.
	pub const fn index(self) -> u8 {
		return self.index;
	}
}

/// Stores vocals.
/// Purely there to make it easier to extract vocals with [`Vocal`].
#[derive(Clone, Copy, Debug)]
pub(super) struct Vocals(&'static str, &'static str);

impl Vocals {
	/// Gets the right vocal with [`Vocal`].
	pub fn get(&self, vocal: Vocal) -> &'static str {
		return match vocal {
			Vocal::First => self.0,
			Vocal::Last => self.1,
		};
	}
}

/// Represents a vocal from [`FORMS_CONFIG`].
#[derive(Clone, Copy, Debug)]
pub(super) enum Vocal {
	/// Represents the first vocal.
	First,
	/// Represents the second vocal.
	Last,
}
