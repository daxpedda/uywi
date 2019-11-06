//! UYWI stems.

use crate::*;

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
	pub(crate) const fn new(concept: Concept) -> Self {
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
	type Item = Word;
	type IntoIter = Words;

	#[must_use]
	fn into_iter(self) -> Self::IntoIter {
		return Words::new(self.concept, self.stem_index);
	}
}
