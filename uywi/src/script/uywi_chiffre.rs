//! UYWI Chiffre.

use super::*;
use crate::*;
use array_init::array_init;
use arrayvec::{ArrayString, ArrayVec};
use std::mem;
use unicode_segmentation::UnicodeSegmentation;

/// Script instantiation. Used to return from enum without [`Box`].
pub(super) const UYWI_CHIFFRE: UywiChiffre = UywiChiffre {};

/// UYWI Chiffre.
#[derive(Clone, Copy, Debug)]
pub(super) struct UywiChiffre {}

impl Extension for UywiChiffre {
	fn script(&self) -> Script {
		return Script::UywiChiffre;
	}

	fn from_concept(&self, string: &str) -> Result<Concept> {
		let length = Length::new(string.grapheme_len())?;

		// store already used radicals
		let mut radicals = ArrayVec::<[_; 4]>::new();

		// iterate through
		for (position, order) in length.radical_order_mirrored().iter().enumerate() {
			// get radical from correct position
			// we checked correct length above
			// returns if invalid radical
			let radical = string.grapheme_nth(usize::from(*order)).expect("no radical found");
			let radical_index = script_radicals()
				.iter()
				.position(|script_radical| return *script_radical == radical)
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

	fn from_str(&self, string: &str) -> Result<ConceptOrWord> {
		let mut concept = ArrayString::<[u8; WORD_BUFFER]>::new();

		if let Ok(concept) = self.from_concept(string) {
			return Ok(ConceptOrWord::Concept(concept));
		} else {
			for length in &[Length::L2, Length::L3, Length::L4] {
				for stem_index in 0..length.stems_per_concept() {
					let stem_index = stem_index.pinto();
					let structure = structure::structures(*length, stem_index);

					if structure.len() == string.grapheme_len() {
						for (letter_structure, letter_concept) in structure.iter().zip(string.graphemes(true)) {
							if let Letter::Consonant(_) = letter_structure {
								concept.try_push_str(letter_concept).map_err(|_| return Error::WordLengthInvalid)?
							}
						}

						if let Ok(concept) = self.from_concept(&concept) {
							for word in Words::new(concept, stem_index) {
								if string == word.to_string(self.script()) {
									return Ok(ConceptOrWord::Word(word));
								}
							}
						}
					}

					concept.clear();
				}
			}

			return Err(Error::WordInvalid);
		}
	}

	fn concept(&self, concept: Concept) -> ArrayString<[u8; CONCEPT_BUFFER]> {
		let mut string = ArrayString::new();

		for radical in concept.radicals() {
			string.push_str(script_radicals()[usize::from(radical.index())]);
		}

		return string;
	}

	fn word(&self, word: Word) -> ArrayString<[u8; WORD_BUFFER]> {
		let concept = word.concept();
		// get concept radicals
		let concept_radicals = concept.radicals();
		// get correct form config
		let vowels = form_configs(concept.length(), word.form_index());
		// get correct structure
		let structure = structure::structures(concept.length(), word.stem_index());

		let mut string = ArrayString::new();

		for letter in structure {
			// print the right letter
			match letter {
				Letter::Consonant(radical_index) | Letter::DuplicateConsonant(radical_index) => {
					let concept_radical_index = usize::from(radical_index);
					let radical_index = usize::from(concept_radicals[concept_radical_index].index());

					string.push_str(script_radicals()[radical_index])
				},
				Letter::Vowel(vowel) | Letter::DuplicateVowel(vowel) | Letter::Nasal(vowel) => string.push_str(vowels.get(vowel)),
			};
		}

		return string;
	}
}

/// Get radicals for this script.
const fn script_radicals() -> [&'static str; NUM_OF_RADICALS] {
	return [
		"?", "Y", "w", "h", "2", "H", "K", "k", "X", "x", "8", "4", "G", "g", "j", "7", "3", "Q", "c", "9", "S", "s", "Z", "z", "D", "d", "T", "t",
		"P", "0", "B", "6", "V", "f", "p", "b", "m", "n", "O", "R", "r", "1", "L", "l",
	];
}

/// Get vowels for this script.
const fn script_vowels() -> [&'static str; 2] {
	return ["o", "ı"];
}

/// List how forms are configured.
fn form_configs(length: Length, form_index: u8) -> Vowels {
	let mut configs = ArrayVec::<[_; 4]>::new();
	let [o, i] = script_vowels();

	match length {
		Length::L2 => configs.try_extend_from_slice(&[Vowels(o, o), Vowels(i, i)]),
		Length::L3 | Length::L4 => configs.try_extend_from_slice(&[Vowels(o, o), Vowels(o, i), Vowels(i, o), Vowels(i, i)]),
	}
	.expect("failed to fill form configs");

	return configs[usize::from(form_index)];
}

/// Stores vowels.
/// Purely there to make it easier to extract vowels with [`Vowel`].
#[derive(Clone, Copy, Debug)]
struct Vowels(&'static str, &'static str);

impl Vowels {
	/// Gets the right vowel with [`Vowel`].
	fn get(&self, vowel: Vowel) -> &'static str {
		return match vowel {
			Vowel::First => self.0,
			Vowel::Last => self.1,
		};
	}
}
