//! Script handling.

mod ipa_peter;
mod uywi_chiffre;

use crate::*;
use arrayvec::ArrayString;
use ipa_peter::IPA_PETER;
use std::fmt::{self, Display, Formatter};
use uywi_chiffre::UYWI_CHIFFRE;

/// Number of possible radicals.
pub const NUM_OF_RADICALS: usize = 44;
/// Size of concept buffer in bytes.
const CONCEPT_BUFFER: usize = 64;
/// Size of word buffer in bytes.
const WORD_BUFFER: usize = 64;

/// Display script.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Script {
	/// UYWI Chiffre.
	UywiChiffre,
	/// IPA (Peter's script)
	IpaPeter,
}

impl Script {
	/// Build [`Concept`] radicals from string.
	/// # Errors
	/// Returns [`Error`] on failing to build [`Concept`] from string.
	#[allow(clippy::wrong_self_convention)]
	pub fn from_concept(self, string: &str) -> Result<Concept> {
		return self.as_ref().from_concept(string);
	}

	/// Build [`Concept`] or [`Word`] from string.
	/// # Errors
	/// Returns [`Error`] on failing to build [`Concept`] or [`Word`] from string.
	#[allow(clippy::wrong_self_convention)]
	pub fn from_str(self, string: &str) -> Result<ConceptOrWord> {
		return self.as_ref().from_str(string);
	}
}

impl Default for Script {
	#[must_use]
	fn default() -> Self {
		return Self::UywiChiffre;
	}
}

impl Display for Script {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		return write!(
			formatter,
			"{}",
			match self {
				Self::UywiChiffre => "Uywi Chiffre",
				Self::IpaPeter => "IPA (Peter's script)",
			}
		);
	}
}

impl AsRef<dyn Extension> for Script {
	#[must_use]
	fn as_ref(&self) -> &(dyn Extension + 'static) {
		return match self {
			Self::UywiChiffre => &UYWI_CHIFFRE,
			Self::IpaPeter => &IPA_PETER,
		};
	}
}

impl Extension for Script {
	fn script(&self) -> Script {
		return self.as_ref().script();
	}

	fn from_concept(&self, string: &str) -> Result<Concept> {
		return self.as_ref().from_concept(string);
	}

	fn from_str(&self, string: &str) -> Result<ConceptOrWord> {
		return self.as_ref().from_str(string);
	}

	fn concept(&self, concept: Concept) -> ArrayString<[u8; CONCEPT_BUFFER]> {
		return self.as_ref().concept(concept);
	}

	fn word(&self, word: Word) -> ArrayString<[u8; WORD_BUFFER]> {
		return self.as_ref().word(word);
	}
}

/// Unify all scripts under a common API.
pub trait Extension {
	/// Get [`Script`].
	fn script(&self) -> Script;

	/// Build [`Concept`] radicals from string.
	fn from_concept(&self, string: &str) -> Result<Concept>;

	/// Build [`Concept`] or [`Word`] from string.
	fn from_str(&self, string: &str) -> Result<ConceptOrWord>;

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
	pub fn to_string(self, script: Script) -> String {
		return match self {
			Self::Concept(concept) => concept.to_string(script),
			Self::Word(word) => word.to_string(script),
		};
	}
}
