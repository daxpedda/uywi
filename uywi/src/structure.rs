//! Structure handling.

use crate::*;
use arrayvec::ArrayVec;

/// Represents a consonant or a vocal in a structure.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Letter {
	/// Represents a consonant position.
	Consonant(u8),
	/// Represents a vocal.
	Vocal(Vocal),
	/// Represents a duplicate.
	Duplicate,
}

impl Letter {
	/// Build new [`Letter::Consonant`].
	pub(crate) fn new_consonant(length: Length, index: u8) -> Self {
		assert!(usize::from(index) < length.as_int(), "consonant index is invalid");

		return Self::Consonant(index);
	}

	/// Build new [`Letter::Vocal`].
	pub(crate) fn new_vocal(index: usize) -> Self {
		return Self::Vocal(match index {
			0 => Vocal::First,
			1 => Vocal::Last,
			_ => unreachable!("vocal index is invalid"),
		});
	}
}

/// Represents a vocal.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Vocal {
	/// Represents the first vocal.
	First,
	/// Represents the second vocal.
	Last,
}

/// List of how stems are configured.
pub(crate) fn structure_list(length: Length) -> ArrayVec<[&'static str; 8]> {
	let mut configs = ArrayVec::new();

	match length {
		Length::L2 => configs.try_extend_from_slice(&["c0 v0 c1", "c0 v0 x c1", "c0 v0 c1 x v0 x"]),
		Length::L3 => configs.try_extend_from_slice(&[
			"c0 v0 c1 v1 c2",
			"c0 v0 c1 x v1 c2",
			"c0 v0 x c1 v1 c2",
			"c0 v0 c1 v1 x c2",
			"c0 v0 c1 v1 c2 x v1 x",
		]),
		Length::L4 => configs.try_extend_from_slice(&[
			"c0 v0 c1 c2 v1 c3",
			"c0 v0 c1 v0 c2 v1 c3",
			"c0 v0 x c1 c2 v1 c3",
			"c0 v0 c1 c2 v1 x c3",
			"c0 v0 c1 c2 v1 c3 x v1 x",
			"c0 v0 c1 v0 x c2 v1 c3",
			"c0 v0 c1 v0 c2 x v1 c3",
			"c0 v0 c1 v0 c2 v1 x c3",
		]),
	}
	.expect("failed to fill stem config internals");

	return configs;
}

/// Stem configuration list.
pub(crate) fn structures(length: Length, stem_index: u8) -> ArrayVec<[Letter; 9]> {
	let mut configs = ArrayVec::<[_; 8]>::new();
	let configs_internal = structure_list(length);

	for index in 0..length.stems_per_concept() {
		let mut config = ArrayVec::new();

		for letter in configs_internal[index].split_whitespace() {
			debug_assert!(
				letter.len() == 1 || letter.len() == 2,
				"configuration option has not exactly 1 or 2 letters"
			);

			let r#type = &letter[0..1];

			match r#type {
				"c" | "v" => {
					let index: u8 = letter[1..2].parse().expect("configuration index isn't a number");

					match r#type {
						// consonant
						"c" => config.push(Letter::new_consonant(length, index)),
						// vocal
						"v" => config.push(Letter::new_vocal(usize::from(index))),
						_ => unreachable!("configuration type not valid"),
					}
				},
				"x" => config.push(Letter::Duplicate),
				_ => unreachable!("configuration type not valid"),
			}
		}

		configs.push(config);
	}

	return configs[usize::from(stem_index)].clone();
}
