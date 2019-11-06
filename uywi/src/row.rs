//! UYWI rows.

use crate::*;

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
	pub(crate) fn new(page: Page) -> Self {
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
