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

mod accent;
mod concept;
mod length;
mod page;
mod row;
mod stem;
mod structure;
mod util;
mod word;

pub use accent::Accent;
pub use concept::Concept;
pub use length::Length;
pub use page::{Page, Pages};

pub(crate) use accent::{AccentExt, NUM_OF_RADICALS};
pub(crate) use concept::Radical;
pub(crate) use row::{Row, Rows};
pub(crate) use stem::{Stem, Stems};
pub(crate) use structure::{Letter, Vocal};
pub(crate) use util::*;
pub(crate) use word::{Word, Words};

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
	/// Error when string to build a [`Page`] was 0.
	#[error("String of page is 0.")]
	PageStringNull,
	/// Error when index to build a [`Concept`] was invalid.
	#[error("Index of concept is invalid.")]
	ConceptIndexInvalid,
	/// Error when string to build a [`Concept`] was invalid.
	#[error("String of concept is invalid.")]
	ConceptStringInvalid,
	/// Error when string to build a [`Concept`] was 0.
	#[error("String of concept is 0.")]
	ConceptStringNull,
	/// Error when string to build a [`Concept`] had duplicate letters.
	#[error("Duplicate letters in a concept are invalid.")]
	ConceptRadicalDuplicate,
	/// Error when string to build a [`Concept`] had invalid letters.
	#[error("Concept contains invalid letters.")]
	ConceptRadicalInvalid,
}
