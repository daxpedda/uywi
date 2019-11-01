#![warn(clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::indexing_slicing,
	clippy::needless_return,
	clippy::non_ascii_literal,
	clippy::missing_inline_in_public_items,
	clippy::option_expect_used,
	clippy::panic,
	clippy::result_expect_used,
	clippy::shadow_reuse,
	clippy::shadow_same
)]

//! UYWI word generator.

mod util;

use array_init::array_init;
use arrayvec::ArrayVec;
use std::{
	fmt::{self, Display, Formatter},
	mem,
	str::FromStr,
};
use util::*;

mod config;

pub use config::Length;
use config::{Letter, Radical};
use thiserror::Error as ThisError;

/// Convenience [`Result`](std::result::Result) replacement.
type Result<T> = std::result::Result<T, Error>;

/// Error type for words.
#[derive(ThisError, Clone, Copy, Debug)]
pub enum Error {
	/// Error when length to build [`Length`] was invalid.
	#[error("Length is invalid.")]
	LengthInvalid,
	/// Error when index to build a [`Page`] was invalid.
	#[error("Index of page is invalid.")]
	PageIndexInvalid,
	/// Error when string to build a [`Page`] was invalid.
	#[error("String of page is invalid.")]
	PageStringInvalid,
	/// Error when index to build a [`Concept`] was invalid.
	#[error("Index of concept is invalid.")]
	ConceptIndexInvalid,
	/// Error when string to build a [`Concept`] was invalid.
	#[error("String of page is invalid.")]
	ConceptStringInvalid,
	/// Error when string to build a [`Concept`] had duplicate letters.
	#[error("Duplicate letters in a concept are invalid.")]
	ConceptRadicalDuplicate,
	/// Error when string to build a [`Concept`] had invalid letters.
	#[error("Concept contains invalid letters.")]
	ConceptRadicalInvalid,
}

/// All pages, only used for iteration.
#[derive(Clone, Debug)]
pub struct Pages {
	/// Concept length.
	length: Length,
	/// Current page index, saved for iteration.
	page_index: usize,
}

impl Pages {
	/// Build [`Pages`] from [`Length`].
	#[must_use]
	pub const fn new(length: Length) -> Self {
		return Self { length, page_index: 0 };
	}
}

impl Iterator for Pages {
	type Item = Page;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we reached the last page
		if self.page_index < self.length.num_of_pages() {
			let page = Some(Page::from_index(self.page_index, self.length).expect("failed to build page"));

			// increment page by one
			self.page_index = self.page_index.padd(1);

			return page;
		} else {
			return None;
		}
	}
}

/// A page, used for type checking.
#[derive(Clone, Copy, Debug)]
pub struct Page {
	/// Page index.
	index: usize,
	/// Concept length.
	length: Length,
}

impl Page {
	/// Build [`Page`] from page index.
	pub fn from_index(index: usize, length: Length) -> Result<Self> {
		if index >= length.num_of_pages() {
			return Err(Error::PageIndexInvalid);
		}

		return Ok(Self { index, length });
	}

	/// Build [`Page`] from page string.
	pub fn from_str(index: &str, length: Length) -> Result<Self> {
		let index: usize = if let Ok(index) = index.parse() {
			index
		} else {
			return Err(Error::PageStringInvalid);
		};

		// in index form a page is always `- 1` to the string
		return Self::from_index(index.psub(1), length);
	}

	/// Get index.
	#[must_use]
	pub const fn index(self) -> usize {
		return self.index;
	}

	/// Get length.
	#[must_use]
	pub const fn length(self) -> Length {
		return self.length;
	}
}

impl IntoIterator for Page {
	type Item = Row;
	type IntoIter = Rows;

	#[must_use]
	fn into_iter(self) -> Self::IntoIter {
		return Rows::new(self);
	}
}

impl Display for Page {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		// in string form a page is always `+ 1` to the index
		return write!(formatter, "{}", self.index.padd(1));
	}
}

/// Rows of a page only used for iteration.
#[derive(Clone, Debug)]
pub struct Rows {
	/// Concept index to start at.
	start_concept_index: usize,
	/// Concept length.
	length: Length,
	/// Current row index, saved for iteration.
	row_index: usize,
}

impl Rows {
	/// Build [`Rows`] from [`Page`].
	fn new(page: Page) -> Self {
		return Self {
			length: page.length(),
			// `PageIndex * ConceptsPerPage`
			start_concept_index: page.index().pmul(page.length().concepts_per_page()),
			row_index: 0,
		};
	}
}

impl Iterator for Rows {
	type Item = Row;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we reached the end of the page
		if self.row_index < self.length.rows_per_page() {
			let row = Some(Row::new(self.row_index, self.start_concept_index, self.length));

			// increment row by one
			self.row_index = self.row_index.padd(1);

			return row;
		} else {
			return None;
		}
	}
}

/// A page row, only used for iteration.
#[derive(Clone, Debug)]
pub struct Row {
	/// Row index.
	index: usize,
	/// Concept index to start at.
	start_concept_index: usize,
	/// Concept length.
	length: Length,
	/// Current iteration, added to `start_concept_index` to get current [`Concept`].
	iter: usize,
}

impl Row {
	/// Build [`Row`] from [`Page`] and row index.
	fn new(index: usize, start_concept_index: usize, length: Length) -> Self {
		assert!(
			start_concept_index.padd(length.concepts_per_row()) < length.num_of_concepts(),
			"concept index in addition to possible concepts needed is higher than number of existing concepts"
		);
		assert!(index < length.rows_per_page(), "row index is higher than number of possible rows");

		return Self {
			index,
			length,
			start_concept_index: start_concept_index.padd(index.pmul(length.concepts_per_row())),
			iter: 0,
		};
	}

	/// Get index.
	#[must_use]
	pub const fn index(&self) -> usize {
		return self.index;
	}
}

impl Iterator for Row {
	type Item = Concept;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we reached the end of the row
		if self.iter < self.length.concepts_per_row() {
			// `StartConcept + Iter`
			let concept = Some(Concept::from_index(self.start_concept_index.padd(self.iter), self.length).expect("invalid concept index"));

			// increment concept by one
			self.iter = self.iter.padd(1);

			return concept;
		} else {
			return None;
		}
	}
}

/// A concept.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Concept {
	/// Concepts are stored as [`Radical`]s.
	radicals: [Radical; 4],
	/// Concept length.
	length: Length,
}

impl Concept {
	/// Build [`Concept`] from concept index.
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
	pub fn from_index_str(index: &str, length: Length) -> Result<Self> {
		let index: usize = if let Ok(index) = index.parse() {
			index
		} else {
			return Err(Error::ConceptStringInvalid);
		};

		// in index form a concept is always `- 1` to the string
		return Self::from_index(index.psub(1), length);
	}

	/// Get radicals.
	fn radicals(self) -> ArrayVec<[Radical; 4]> {
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

impl FromStr for Concept {
	type Err = Error;

	fn from_str(string: &str) -> Result<Self> {
		// check if concept has the correct length
		let length = Length::new(string.grapheme_len())?;

		// store already used radicals
		let mut radicals = ArrayVec::<[_; 4]>::new();

		// iterate through
		for (position, order) in length.radical_order_mirrored().iter().enumerate() {
			// get radical from correct position
			// we checked correct length above
			// returns if invalid radical
			let radical = Radical::from_str(string.grapheme_nth((*order).into()).expect("no radical found in given position"))?;

			// get radicals in appropriate order for this position and filter already used radicals
			let mut radicals_ordered = length.radicals_ordered(position, &radicals);

			// check if radical is valid
			if !radicals_ordered.any(|ordered_radical| return ordered_radical == radical) {
				// if not found, radical has to be duplicate and already used
				// we already checked for invalid radicals above
				return Err(Error::ConceptRadicalDuplicate);
			}

			// drop the iterator because we need a `&mut` to `radicals_used`
			mem::drop(radicals_ordered);

			// store found radical
			radicals.push(radical);
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

		return Ok(Self { length, radicals });
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

impl Display for Concept {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		for radical in &self.radicals() {
			write!(formatter, "{}", radical.as_str())?;
		}

		return Ok(());
	}
}

/// Stems, only used for iteration.
#[derive(Clone, Debug)]
pub struct Stems {
	/// [`Concept`] to build [`Stem`] from.
	concept: Concept,
	/// Current stem index, saved for iteration.
	stem_index: u8,
}

impl Stems {
	/// Build new [`Stems`].
	const fn new(concept: Concept) -> Self {
		return Self { concept, stem_index: 0 };
	}
}

impl Iterator for Stems {
	type Item = Stem;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we finished with all stems
		if usize::from(self.stem_index) < self.concept.length().stems_per_concept() {
			let stem = Some(Stem::new(self.concept, self.stem_index));

			// increment stem by one
			self.stem_index = self.stem_index.padd(1);

			return stem;
		} else {
			return None;
		};
	}
}

/// A stem, only used for iteration.
#[derive(Clone, Copy, Debug)]
pub struct Stem {
	/// [`Concept`] to build from.
	concept: Concept,
	/// Stem index.
	stem_index: u8,
}

impl Stem {
	/// Build new [`Stem`].
	fn new(concept: Concept, index: u8) -> Self {
		assert!(
			usize::from(index) < concept.length().stems_per_concept(),
			"stem index is higher than number of possible stems"
		);

		return Self { concept, stem_index: index };
	}
}

impl IntoIterator for Stem {
	type Item = Form;
	type IntoIter = Forms;

	#[must_use]
	fn into_iter(self) -> Self::IntoIter {
		return Forms::new(self.concept, self.stem_index);
	}
}

/// Forms, only used for iteration.
#[derive(Clone, Debug)]
pub struct Forms {
	/// [`Concept`] to build [`Form`] from.
	concept: Concept,
	/// Stem index.
	stem_index: u8,
	/// Current form index, saved for iteration.
	form_index: u8,
}

impl Forms {
	/// Build new [`Forms`].
	const fn new(concept: Concept, stem_index: u8) -> Self {
		return Self {
			concept,
			stem_index,
			form_index: 0,
		};
	}
}

impl Iterator for Forms {
	type Item = Form;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we finished with all forms
		if usize::from(self.form_index) < self.concept.length().forms_per_stem() {
			let form = Some(Form::new(self.concept, self.stem_index, self.form_index));

			// increment form by one
			self.form_index = self.form_index.padd(1);

			return form;
		} else {
			return None;
		}
	}
}

/// A form.
#[derive(Clone, Copy, Debug)]
pub struct Form {
	/// [`Concept`] to build from.
	concept: Concept,
	/// Stem configuration.
	stem_index: u8,
	/// Represents which form index this form is.
	index: u8,
}

impl Form {
	/// Build new [`Form`].
	fn new(concept: Concept, stem_index: u8, index: u8) -> Self {
		assert!(
			usize::from(stem_index) < concept.length().stems_per_concept(),
			"stem index is higher than number of possible stems"
		);
		assert!(
			usize::from(index) < concept.length().forms_per_stem(),
			"form index is higher than number of possible stems"
		);

		return Self { concept, stem_index, index };
	}
}

impl Display for Form {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		// get correct stem config
		let stem_configs = &self.concept.length().stem_configs()[usize::from(self.stem_index)];
		// get concept radicals
		let radicals = self.concept.radicals();
		// get correct form config
		let vocals = &self.concept.length().forms_config()[usize::from(self.index)];

		for letter in stem_configs {
			// print the right letter
			match letter {
				Letter::Radical(radical_index) => write!(formatter, "{}", radicals[usize::from(*radical_index)].as_str())?,
				Letter::Vocal(vocal) => write!(formatter, "{}", vocals.get(*vocal))?,
			}
		}

		return Ok(());
	}
}
