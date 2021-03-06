//! Structure handling.

use crate::*;
use arrayvec::ArrayVec;

/// Represents a consonant or a vowel in a structure.
#[derive(Clone, Copy, Debug)]
pub enum Letter {
	/// Represents a consonant position.
	Consonant(u8),
	/// Represents a vowel.
	Vowel(Vowel),
	/// Represents a duplicate consonant.
	DuplicateConsonant(u8),
	/// Represents a duplicate vowel.
	DuplicateVowel(Vowel),
	/// Represents a nasal vowel.
	Nasal(Vowel),
}

impl Letter {
	/// Build new [`Letter::Consonant`].
	pub(crate) fn new_consonant(length: Length, index: u8) -> Self {
		assert!(usize::from(index) < length.as_int(), "consonant index is invalid");

		return Self::Consonant(index);
	}

	/// Build new [`Letter::Vowel`].
	pub(crate) fn new_vowel(index: usize) -> Self {
		return Self::Vowel(match index {
			0 => Vowel::First,
			1 => Vowel::Last,
			_ => unreachable!("vowel index is invalid"),
		});
	}

	/// Build new [`Letter::DuplicateConsonant`].
	pub(crate) fn new_duplicate_consonant(length: Length, index: u8) -> Self {
		assert!(usize::from(index) < length.as_int(), "consonant index is invalid");

		return Self::DuplicateConsonant(index);
	}

	/// Build new [`Letter::DuplicateVowel`].
	pub(crate) fn new_duplicate_vowel(index: usize) -> Self {
		return Self::DuplicateVowel(match index {
			0 => Vowel::First,
			1 => Vowel::Last,
			_ => unreachable!("vowel index is invalid"),
		});
	}

	/// Build new [`Letter::Nasal`].
	pub(crate) fn new_nasal(index: usize) -> Self {
		return Self::Nasal(match index {
			0 => Vowel::First,
			1 => Vowel::Last,
			_ => unreachable!("vowel index is invalid"),
		});
	}
}

/// Represents a vowel.
#[derive(Clone, Copy, Debug)]
pub enum Vowel {
	/// Represents the first vowel.
	First,
	/// Represents the second vowel.
	Last,
}

/// List of how stems are configured.
pub fn list(length: Length) -> ArrayVec<[&'static str; 8]> {
	let mut configs = ArrayVec::new();

	match length {
		Length::L2 => configs.try_extend_from_slice(&["c0 v0 c1", "c0 v0 xv0 c1", "c0 v0 c1 xc1 xn0"]),
		Length::L3 => configs.try_extend_from_slice(&[
			"c0 v0 c1 v1 c2",
			"c0 v0 c1 xc1 v1 c2",
			"c0 v0 xv0 c1 v1 c2",
			"c0 v0 c1 v1 xv1 c2",
			"c0 v0 c1 v1 c2 xc2 xn1",
		]),
		Length::L4 => configs.try_extend_from_slice(&[
			"c0 v0 c1 c2 v1 c3",
			"c0 v0 c1 v0 c2 v1 c3",
			"c0 v0 xv0 c1 c2 v1 c3",
			"c0 v0 c1 c2 v1 xv1 c3",
			"c0 v0 c1 c2 v1 c3 xc3 xn1",
			"c0 v0 c1 v0 xv0 c2 v1 c3",
			"c0 v0 c1 v0 c2 xc2 v1 c3",
			"c0 v0 c1 v0 c2 v1 xv1 c3",
		]),
	}
	.expect("failed to fill stem config internals");

	return configs;
}

/// Stem configuration list.
pub fn structures(length: Length, stem_index: u8) -> ArrayVec<[Letter; 8]> {
	let mut configs = ArrayVec::<[_; 8]>::new();
	let configs_internal = list(length);

	for index in 0..length.stems_per_concept() {
		let mut config = ArrayVec::new();

		for letter in configs_internal[index].split_whitespace() {
			debug_assert!(
				letter.len() == 2 || letter.len() == 3,
				"configuration option has not exactly 2 or 3 letters"
			);

			let r#type = &letter[0..1];

			match r#type {
				"c" | "v" => {
					let index: u8 = letter[1..2].parse().expect("configuration index isn't a number");

					match r#type {
						// consonant
						"c" => config.push(Letter::new_consonant(length, index)),
						// vowel
						"v" => config.push(Letter::new_vowel(usize::from(index))),
						_ => unreachable!("configuration type not valid"),
					}
				},
				"x" => {
					let r#type = &letter[1..2];
					let index: u8 = letter[2..3].parse().expect("configuration index isn't a number");

					match r#type {
						// consonant
						"c" => config.push(Letter::new_duplicate_consonant(length, index)),
						// vowel
						"v" => config.push(Letter::new_duplicate_vowel(usize::from(index))),
						// nasal
						"n" => config.push(Letter::new_nasal(usize::from(index))),
						_ => unreachable!("configuration type not valid"),
					}
				},
				_ => unreachable!("configuration type not valid"),
			}
		}

		configs.push(config);
	}

	return configs[usize::from(stem_index)].clone();
}
