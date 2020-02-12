//! UYWI words.

use crate::*;

/// Words, only used for iteration.
#[derive(Clone, Debug)]
pub struct Words {
	/// [`Concept`] to build [`Word`] from.
	concept: Concept,
	/// Stem index.
	stem_index: u8,
	/// Current word index, saved for iteration.
	form_index: u8,
}

impl Words {
	/// Build new [`Words`].
	pub(crate) const fn new(concept: Concept, stem_index: u8) -> Self {
		return Self {
			concept,
			stem_index,
			form_index: 0,
		};
	}
}

impl Iterator for Words {
	type Item = Word;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we finished with all words
		if usize::from(self.form_index) < self.concept.length().words_per_stem() {
			let word = Some(Word::new(self.concept, self.stem_index, self.form_index));

			// increment form by one
			self.form_index = self.form_index.padd(1);

			return word;
		} else {
			return None;
		}
	}
}

/// A word.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Word {
	/// [`Concept`] to build from.
	concept: Concept,
	/// Stem configuration.
	stem_index: u8,
	/// Represents which form index this word is.
	form_index: u8,
}

impl Word {
	/// Build new [`Word`].
	fn new(concept: Concept, stem_index: u8, form_index: u8) -> Self {
		assert!(
			usize::from(stem_index) < concept.length().stems_per_concept(),
			"stem index is higher than number of possible stems"
		);
		assert!(
			usize::from(form_index) < concept.length().words_per_stem(),
			"form index is higher than number of possible forms"
		);

		return Self {
			concept,
			stem_index,
			form_index,
		};
	}

	/// Get word as [`String`].
	#[must_use]
	pub fn to_string(self, accent: Accent) -> String {
		return accent.word(self).to_string();
	}

	/// Get [`Concept`].
	pub const fn concept(self) -> Concept {
		return self.concept;
	}

	/// Get stem index.
	pub const fn stem_index(self) -> u8 {
		return self.stem_index;
	}

	/// Get form index.
	pub const fn form_index(self) -> u8 {
		return self.form_index;
	}
}
