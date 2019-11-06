//! UYWI Chiffre.

use super::*;
use crate::*;
use array_init::array_init;
use arrayvec::{ArrayString, ArrayVec};
use std::mem;

/// Accent instantiation. Used to return from enum without [`Box`].
pub(crate) const UYWI_CHIFFRE: UywiChiffre = UywiChiffre {};

/// UYWI Chiffre.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UywiChiffre {}

impl AccentExt for UywiChiffre {
	fn build_concept(&self, string: &str) -> Result<Concept> {
		let length = Length::new(string.grapheme_len())?;

		// store already used radicals
		let mut radicals = ArrayVec::<[_; 4]>::new();

		// iterate through
		for (position, order) in length.radical_order_mirrored().iter().enumerate() {
			// get radical from correct position
			// we checked correct length above
			// returns if invalid radical
			let radical = string.grapheme_nth(usize::from(*order)).expect("no radical found");
			let radical_index = accent_radicals()
				.iter()
				.position(|accent_radical| return *accent_radical == radical)
				.ok_or(Error::ConceptRadicalInvalid)?;
			let radical = Radical::from_index(radical_index.pinto());

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

		return Ok(Concept::new(radicals, length));
	}

	fn build_concept_string(&self, concept: Concept) -> ArrayString<[u8; 64]> {
		let mut string = ArrayString::new();

		for radical in concept.radicals() {
			string.push_str(accent_radicals()[usize::from(radical.index())]);
		}

		return string;
	}

	fn build_word(&self, concept: Concept, stem_index: u8, form_index: u8) -> ArrayString<[u8; 64]> {
		// get concept radicals
		let concept_radicals = concept.radicals();
		// get correct form config
		let vocals = form_configs(concept.length(), form_index);
		// get correct structure
		let structure = &mut structure::structures(concept.length(), stem_index);

		// search for a `x v* x` pattern in a structure and remove last `x` if found
		for index in 0..structure.len().psub(2) {
			if let Letter::Duplicate = structure[index] {
				if let Letter::Vocal(..) = structure[index.padd(1)] {
					if let Letter::Duplicate = structure[index.padd(2)] {
						structure.pop();
					}
				}
			}
		}

		let mut string = ArrayString::new();

		let mut last_letter = None;

		for letter in structure {
			// print the right letter
			last_letter = match letter {
				Letter::Consonant(radical_index) => {
					let concept_radical_index = usize::from(*radical_index);
					let radical_index = usize::from(concept_radicals[concept_radical_index].index());

					Some(accent_radicals()[radical_index])
				},
				Letter::Vocal(vocal) => Some(vocals.get(*vocal)),
				Letter::Duplicate => last_letter,
			};

			string.push_str(last_letter.expect("no last letter found"));
		}

		return string;
	}
}

/// Get radicals for this accent.
const fn accent_radicals() -> [&'static str; NUM_OF_RADICALS] {
	return [
		"?", "Y", "w", "h", "2", "H", "K", "k", "X", "x", "8", "4", "G", "g", "j", "7", "3", "Q", "c", "9", "S", "s", "Z", "z", "D", "d", "T", "t",
		"P", "0", "B", "6", "V", "f", "p", "b", "m", "n", "O", "R", "r", "1", "L", "l",
	];
}

/// List how forms are configured.
fn form_configs(length: Length, form_index: u8) -> Vocals {
	let mut configs = ArrayVec::<[_; 4]>::new();

	match length {
		Length::L2 => configs.try_extend_from_slice(&[Vocals("o", "o"), Vocals("ı", "ı")]),
		Length::L3 | Length::L4 => configs.try_extend_from_slice(&[Vocals("o", "o"), Vocals("o", "ı"), Vocals("ı", "o"), Vocals("ı", "ı")]),
	}
	.expect("failed to fill form configs");

	return configs[usize::from(form_index)];
}

/// Stores vocals.
/// Purely there to make it easier to extract vocals with [`Vocal`].
#[derive(Clone, Copy, Debug)]
struct Vocals(pub(crate) &'static str, pub(crate) &'static str);

impl Vocals {
	/// Gets the right vocal with [`Vocal`].
	pub fn get(&self, vocal: Vocal) -> &'static str {
		return match vocal {
			Vocal::First => self.0,
			Vocal::Last => self.1,
		};
	}
}
