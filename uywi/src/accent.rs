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

/// Display accent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accent {
	/// UYWI Chiffre.
	UywiChiffre,
	/// IPA (Peter's accent)
	IpaPeter,
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
	fn build_concept(&self, string: &str) -> Result<Concept> {
		return self.as_ref().build_concept(string);
	}

	fn build_concept_string(&self, concept: Concept) -> ArrayString<[u8; 64]> {
		return self.as_ref().build_concept_string(concept);
	}

	fn build_word(&self, concept: Concept, stem_index: u8, form_index: u8) -> ArrayString<[u8; 64]> {
		return self.as_ref().build_word(concept, stem_index, form_index);
	}
}

/// Unify all accents under a common API.
pub(crate) trait AccentExt {
	/// Build concept radicals from string.
	fn build_concept(&self, string: &str) -> Result<Concept>;

	/// Build concept string.
	fn build_concept_string(&self, concept: Concept) -> ArrayString<[u8; 64]>;

	/// Build word.
	fn build_word(&self, concept: Concept, stem_index: u8, form_index: u8) -> ArrayString<[u8; 64]>;
}
