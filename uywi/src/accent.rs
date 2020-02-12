//! Accent handling.

mod ipa_peter;
mod uywi_chiffre;

use crate::*;
use arrayvec::ArrayString;
use ipa_peter::IPA_PETER;
use std::fmt::{self, Display, Formatter};
use uywi_chiffre::UYWI_CHIFFRE;

/// Number of possible radicals.
pub(crate) const NUM_OF_RADICALS: usize = 44;
/// Size of concept buffer in bytes.
const CONCEPT_BUFFER: usize = 64;
/// Size of word buffer in bytes.
const WORD_BUFFER: usize = 64;

/// Display accent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accent {
	/// UYWI Chiffre.
	UywiChiffre,
	/// IPA (Peter's accent)
	IpaPeter,
}

impl Accent {
	/// Build [`Concept`] or [`Word`] from string.
	/// # Errors
	/// Returns [`Error`] on failing to build [`Concept`] or [`Word`] from [`String`].
	#[allow(clippy::wrong_self_convention)]
	pub fn from_string(self, string: &str) -> Result<ConceptOrWord> {
		return self.as_ref().from_string(string);
	}
}

impl Default for Accent {
	#[must_use]
	fn default() -> Self {
		return Self::UywiChiffre;
	}
}

impl Display for Accent {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		return write!(
			formatter,
			"{}",
			match self {
				Self::UywiChiffre => "Uywi Chiffre",
				Self::IpaPeter => "IPA (Peter's accent)",
			}
		);
	}
}

impl AsRef<dyn AccentExt> for Accent {
	#[must_use]
	fn as_ref(&self) -> &(dyn AccentExt + 'static) {
		return match self {
			Self::UywiChiffre => &UYWI_CHIFFRE,
			Self::IpaPeter => &IPA_PETER,
		};
	}
}

impl AccentExt for Accent {
	fn accent(&self) -> Accent {
		return self.as_ref().accent();
	}

	fn from_concept(&self, string: &str) -> Result<Concept> {
		return self.as_ref().from_concept(string);
	}

	fn from_string(&self, string: &str) -> Result<ConceptOrWord> {
		return self.as_ref().from_string(string);
	}

	fn concept(&self, concept: Concept) -> ArrayString<[u8; CONCEPT_BUFFER]> {
		return self.as_ref().concept(concept);
	}

	fn word(&self, word: Word) -> ArrayString<[u8; WORD_BUFFER]> {
		return self.as_ref().word(word);
	}
}

/// Unify all accents under a common API.
pub(crate) trait AccentExt {
	/// Get [`Accent`].
	fn accent(&self) -> Accent;

	/// Build [`Concept`] radicals from string.
	fn from_concept(&self, string: &str) -> Result<Concept>;

	/// Build [`Concept`] or [`Word`] from string.
	fn from_string(&self, string: &str) -> Result<ConceptOrWord>;

	/// Build concept string.
	fn concept(&self, concept: Concept) -> ArrayString<[u8; CONCEPT_BUFFER]>;

	/// Build word.
	fn word(&self, word: Word) -> ArrayString<[u8; WORD_BUFFER]>;
}

/// Represents a [`Concept`] or [`Word`], return type from [`from_word`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConceptOrWord {
	/// A [`Concept`].
	Concept(Concept),
	/// A [`Word`].
	Word(Word),
}

impl ConceptOrWord {
	/// Get string from [`Concept`] or [`Word`].
	pub fn to_string(self, accent: Accent) -> String {
		return match self {
			Self::Concept(concept) => concept.to_string(accent),
			Self::Word(word) => word.to_string(accent),
		};
	}
}
