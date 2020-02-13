//! IPA (Peter's script)

use super::*;
use crate::*;
use arrayvec::{ArrayString, ArrayVec};

/// Script instantiation. Used to return from enum without [`Box`].
pub(super) const IPA_PETER: IpaPeter = IpaPeter {};

/// UYWI Chiffre.
#[derive(Clone, Copy, Debug)]
pub(super) struct IpaPeter {}

impl ScriptExt for IpaPeter {
	fn script(&self) -> Script {
		return Script::IpaPeter;
	}

	fn from_concept(&self, _string: &str) -> Result<Concept> {
		unimplemented!("build concept for ipa peter is unimplemented")
	}

	fn from_str(&self, _string: &str) -> Result<ConceptOrWord> {
		unimplemented!("build word for ipa peter is unimplemented")
	}

	fn concept(&self, concept: Concept) -> ArrayString<[u8; CONCEPT_BUFFER]> {
		let mut string = ArrayString::new();

		for radical in concept.radicals() {
			let radical = script_radicals()[usize::from(radical.index())];
			string.push_str(&radical.as_str(true, false));
		}

		return string;
	}

	fn word(&self, word: Word) -> ArrayString<[u8; WORD_BUFFER]> {
		let concept = word.concept();
		// get correct structure
		let structure = structure::structures(concept.length(), word.stem_index());
		// save ipa specfici structure
		let mut ipa_structure = ArrayVec::<[_; 8]>::new();

		word_base(&structure, concept, word.form_index(), &mut ipa_structure);
		assimilation_1(&mut ipa_structure);
		assimilation_2(&mut ipa_structure);
		assimilation_3(&mut ipa_structure);
		assimilation_4(&mut ipa_structure);
		assimilation_5(&structure, &mut ipa_structure);
		assimilation_6(&structure, &mut ipa_structure);
		assimilation_7(&mut ipa_structure);

		// build word
		let mut word = ArrayString::new();

		for letter_ipa in ipa_structure {
			match letter_ipa {
				IpaLetter::Radical(radical, alternative) => word.push_str(&radical.as_str(false, alternative)),
				IpaLetter::Vowel(vowel) => word.push_str(vowel.as_str()),
				IpaLetter::Duplicate => word.push_str("ː"),
				IpaLetter::Removed => (),
			}
		}

		return word;
	}
}

/// Build base of the word: insert concept radicals, fix stem and form structure and insert neutral vowels.
fn word_base(structure: &ArrayVec<[Letter; 8]>, concept: Concept, form_index: u8, ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	// get concept radicals
	let concept_radicals = concept.radicals();
	// get correct form config
	let vowels = form_configs(concept.length(), form_index);

	for letter_structure in structure {
		match letter_structure {
			Letter::Consonant(radical_index) => {
				let concept_radical_index = usize::from(*radical_index);
				let radical_index = usize::from(concept_radicals[concept_radical_index].index());
				let radical = script_radicals()[radical_index];

				ipa_structure.push(IpaLetter::Radical(radical, false));
			},
			Letter::Vowel(vowel) | Letter::Nasal(vowel) => {
				let vowel = vowels.get(*vowel);
				ipa_structure.push(IpaLetter::Vowel(vowel));
			},
			Letter::DuplicateConsonant(..) | Letter::DuplicateVowel(..) => {
				ipa_structure.push(IpaLetter::Duplicate);
			},
		}
	}
}

/// First assimilation: turn all vowels around dark consonants to dark vowels.
fn assimilation_1(ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (position, letter_ipa) in ipa_structure.clone().into_iter().enumerate() {
		// check if this is a radical
		if let IpaLetter::Radical(letter, _) = letter_ipa {
			// check if its quality is dark
			if letter.is_dark() {
				// if there is something before it check if it's a vowel
				if position > 0 {
					let mut position = position.psub(1);

					// if duplicate check the one before it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.psub(1);
					}

					// turn it dark
					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_dark()
					}
				}

				// if there is something after it check if it's a vowel
				if position.padd(1) < ipa_structure.len() {
					let mut position = position.padd(1);

					// if duplicate check the one after it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.padd(1);
					}

					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_dark()
					}
				}
			}
		}
	}
}

/// Second assimilation: turn all vowels around light consonants to light vowels.
fn assimilation_2(ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (position, letter_ipa) in ipa_structure.clone().into_iter().enumerate() {
		// check if this is a radical
		if let IpaLetter::Radical(letter, _) = letter_ipa {
			// check if its quality is light
			if letter.is_light() {
				// if there is something before it check if it's a vowel
				if position > 0 {
					let mut position = position.psub(1);

					// if duplicate check the one before it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.psub(1);
					}

					// turn it light
					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_light()
					}
				}

				// if there is something after it check if it's a vowel
				if position.padd(1) < ipa_structure.len() {
					let mut position = position.padd(1);

					// if duplicate check the one after it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.padd(1);
					}

					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_light()
					}
				}
			}
		}
	}
}

/// Third assimilation: turn all vowels after shading consonants to shading vowels.
fn assimilation_3(ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (position, letter_ipa) in ipa_structure.clone().into_iter().enumerate() {
		// check if this is a radical
		if let IpaLetter::Radical(letter, _) = letter_ipa {
			// check if its shading
			if letter.is_shading() {
				// if there is something after it check if it's a vowel
				if position.padd(1) < ipa_structure.len() {
					let mut position = position.padd(1);

					// if duplicate check the one after it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.padd(1);
					}

					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_shading()
					}
				}
			}
		}
	}
}

/// Fifth assimilation: turn all consonants around voiceless consonants that are [`Alternative`](Quality3::Alternative) to their alternative form.
fn assimilation_4(ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (position, letter_ipa) in ipa_structure.clone().into_iter().enumerate() {
		// check if this is a radical
		if let IpaLetter::Radical(letter, _) = letter_ipa {
			// check if voiceless
			if letter.is_voiceless() {
				// if there is something before it check if it's a consonant
				if position > 0 {
					// turn consonant to alternative
					if let IpaLetter::Radical(letter, alternative) = &mut ipa_structure[position.psub(1)] {
						if letter.alternative().is_some() {
							*alternative = true;
						}
					}
				}

				// if there is something after it check if it's a consonant
				if position.padd(1) < ipa_structure.len() {
					if let IpaLetter::Radical(letter, alternative) = &mut ipa_structure[position.padd(1)] {
						if letter.alternative().is_some() {
							*alternative = true;
						}
					}
				}
			}
		}
	}
}

/// Sixth assimilation: if the consonant should be removed at the beginning or end, remove it.
fn assimilation_5(structure: &ArrayVec<[Letter; 8]>, ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	{
		// filter only for radicals and get the first one
		let position = structure
			.iter()
			.enumerate()
			.find_map(|(position, letter)| {
				if let Letter::Consonant(..) = letter {
					return Some(position);
				} else {
					return None;
				}
			})
			.expect("no consonant found");

		let letter = &mut ipa_structure[position];

		if let IpaLetter::Radical(radical, _) = letter {
			// remove if consonant is beginning
			if radical.is_beginning() {
				*letter = IpaLetter::Removed;
			}
		} else {
			unreachable!("ipa and letter structure don't match");
		}
	}

	{
		// filter only for radicals and get the last one
		let position = structure
			.iter()
			.enumerate()
			.filter_map(|(position, letter)| {
				if let Letter::Consonant(..) = letter {
					return Some(position);
				} else {
					return None;
				}
			})
			.last()
			.expect("no consonant found");

		let letter = &mut ipa_structure[position];

		if let IpaLetter::Radical(radical, _) = letter {
			// remove if consonant is end
			if radical.is_end() {
				*letter = IpaLetter::Removed;

				// remove all letters afterwards except the nasal
				for letter in ipa_structure
					.iter_mut()
					.zip(structure)
					.skip(position)
					.filter_map(|(letter_ipa, letter_structure)| {
						if let Letter::Nasal(..) = letter_structure {
							return None;
						} else {
							return Some(letter_ipa);
						}
					}) {
					*letter = IpaLetter::Removed;
				}

				let mut nasal_found = None;

				// if the last letter before the nasal is a vowel, remove it
				for (letter_ipa, letter_structure) in ipa_structure.iter_mut().zip(structure).rev() {
					// if we find the nasal, save that we found it
					if let Letter::Nasal(..) = letter_structure {
						nasal_found = Some(letter_ipa);
						continue;
					}

					match letter_ipa {
						// if we found a vowel
						IpaLetter::Vowel(..) => {
							// check if we already found the nasal
							if let Some(nasal_found) = nasal_found {
								// assign found vowel to nasal
								*nasal_found = *letter_ipa;
								*letter_ipa = IpaLetter::Removed;
								break;
							// otherwise there was no nasal
							} else {
								break;
							}
						},
						// if we find anything else than `Removed` we break
						IpaLetter::Removed => continue,
						_ => break,
					}
				}
			}
		} else {
			unreachable!("ipa and letter structure don't match");
		}
	}
}

/// Seventh assimilation: turn vowels that should be nasal to nasal.
fn assimilation_6(structure: &ArrayVec<[Letter; 8]>, ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (letter_structure, letter_ipa) in structure.iter().zip(ipa_structure) {
		// check if this is a nasal
		if let Letter::Nasal(..) = letter_structure {
			if let IpaLetter::Vowel(vowel) = letter_ipa {
				vowel.as_nasal();
			} else {
				unreachable!("ipa and letter structure don't match");
			}
		}
	}
}

/// Fourth assimilation: turn all vowels after rounding consonants to rounding vowels.
fn assimilation_7(ipa_structure: &mut ArrayVec<[IpaLetter; 8]>) {
	for (position, letter_ipa) in ipa_structure.clone().into_iter().enumerate() {
		// check if this is a radical
		if let IpaLetter::Radical(letter, _) = letter_ipa {
			// check if its rounding
			if letter.is_rounding() {
				// if there is something after it check if it's a vowel
				if position.padd(1) < ipa_structure.len() {
					let mut position = position.padd(1);

					// if duplicate check the one after it
					if let IpaLetter::Duplicate = ipa_structure[position] {
						position = position.padd(1);
					}

					if let IpaLetter::Vowel(vowel) = &mut ipa_structure[position] {
						vowel.as_rounding()
					}
				}
			}
		}
	}
}

/// List of radicals with all exceptions.
const fn script_radicals() -> [IpaRadical; NUM_OF_RADICALS] {
	use Quality1::*;
	use Quality2::*;
	use Quality3::*;
	use Quality4::*;

	#[rustfmt::skip]
	return [
        IpaRadical("ʔ",  Neutral, false, None,           None,                   Some(Beginning)),
        IpaRadical("j",  Dark,    false, None,           None,                   None),
        IpaRadical("w",  Neutral, false, None,           None,                   None),
        IpaRadical("h",  Neutral, false, None,           Some(Voiceless),        Some(End)),
        IpaRadical("ʕ",  Neutral, false, None,           None,                   None),
        IpaRadical("ħ",  Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("k",  Dark,    false, Some(Shading),  Some(Voiceless),        None),
        IpaRadical("kʰ", Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("x",  Dark,    false, Some(Shading),  Some(Voiceless),        None),
        IpaRadical("x",  Light,   false, None,           Some(Voiceless),        None),
        IpaRadical("ʁ",  Neutral, false, None,           Some(Alternative("χ")), None),
        IpaRadical("ɟ",  Neutral, false, None,           Some(Alternative("c")), None),
        IpaRadical("g",  Dark,    false, Some(Shading),  None,                   None),
        IpaRadical("g",  Light,   false, None,           None,                   None),
        IpaRadical("ɥ",  Light,   false, Some(Rounding), None,                   None),
        IpaRadical("d͡ʐ", Dark,    false, Some(Shading),  None,                   None),
        IpaRadical("d͡ʒ", Neutral, false, None,           None,                   None),
        IpaRadical("ʂ",  Dark,    false, Some(Shading),  Some(Voiceless),        None),
        IpaRadical("ɕ",  Light,   false, None,           Some(Voiceless),        None),
        IpaRadical("ʃ",  Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("s",  Dark,    false, Some(Shading),  Some(Voiceless),        None),
        IpaRadical("s",  Light,   true,  None,           Some(Voiceless),        None),
        IpaRadical("z",  Dark,    false, Some(Shading),  None,                   None),
        IpaRadical("z",  Light,   true,  None,           None,                   None),
        IpaRadical("d",  Dark,    false, Some(Shading),  None,                   None),
        IpaRadical("d",  Light,   true,  None,           None,                   None),
        IpaRadical("t",  Dark,    false, Some(Shading),  Some(Voiceless),        None),
        IpaRadical("tʰ", Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("t͡ɕ", Light,   false, None,           Some(Voiceless),        None),
        IpaRadical("t͡s", Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("θ",  Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("ð",  Neutral, false, None,           None,                   None),
        IpaRadical("v",  Dark,    false, None,           None,                   None),
        IpaRadical("f",  Neutral, false, None,           Some(Voiceless),        None),
        IpaRadical("p",  Neutral, false, Some(Rounding), Some(Voiceless),        None),
        IpaRadical("b",  Neutral, false, Some(Rounding), None,                   None),
        IpaRadical("m",  Neutral, false, Some(Rounding), None,                   None),
        IpaRadical("n",  Neutral, false, None,           None,                   None),
        IpaRadical("ŋ",  Neutral, false, None,           None,                   None),
        IpaRadical("ɻ",  Dark,    false, Some(Rounding), None,                   None),
        IpaRadical("r",  Neutral, false, None,           None,                   None),
        IpaRadical("l",  Light,   true,  None,           None,                   None),
        IpaRadical("ʟ",  Dark,    false, Some(Shading),  None,                   None),
        IpaRadical("l",  Neutral, false, None,           None,                   None),
	];
}

/// List how forms are configured.
fn form_configs(length: Length, form_index: u8) -> Vowels {
	use IpaVowel::*;

	let mut configs = ArrayVec::<[_; 4]>::new();

	match length {
		Length::L2 => configs.try_extend_from_slice(&[Vowels(NeutralA, NeutralA), Vowels(NeutralI, NeutralI)]),
		Length::L3 | Length::L4 => configs.try_extend_from_slice(&[
			Vowels(NeutralA, NeutralE),
			Vowels(NeutralE, NeutralI),
			Vowels(NeutralU, NeutralA),
			Vowels(NeutralU, NeutralI),
		]),
	}
	.expect("failed to fill form configs");

	return configs[usize::from(form_index)];
}

/// Save radicals with all the exceptions.
#[derive(Clone, Copy, Debug)]
struct IpaRadical(&'static str, Quality1, bool, Option<Quality2>, Option<Quality3>, Option<Quality4>);

impl IpaRadical {
	/// Get radical in string form.
	fn as_str(self, force_quality: bool, alternative: bool) -> ArrayString<[u8; 8]> {
		// start with original or with alternative
		let mut string = ArrayString::from({
			if alternative {
				self.alternative().expect("no alternative found")
			} else {
				self.0
			}
		})
		.expect("failed to turn radical to string");

		// sow quality if forced or part of the letter
		if force_quality || self.2 {
			string.push_str(self.quality().as_str())
		}

		return string;
	}

	/// Get radical's quality.
	const fn quality(self) -> Quality1 {
		return self.1;
	}

	/// Get if radical is dark.
	fn is_dark(self) -> bool {
		if let Quality1::Dark = self.1 {
			return true;
		} else {
			return false;
		}
	}

	/// Get if radical is light.
	fn is_light(self) -> bool {
		if let Quality1::Light = self.1 {
			return true;
		} else {
			return false;
		}
	}

	/// Get if radical is shading.
	fn is_shading(self) -> bool {
		if let Some(Quality2::Shading) = self.3 {
			return true;
		} else {
			return false;
		}
	}

	/// Get if radical is rounding.
	fn is_rounding(self) -> bool {
		if let Some(Quality2::Rounding) = self.3 {
			return true;
		} else {
			return false;
		}
	}

	/// Get radical's alternative.
	fn is_voiceless(self) -> bool {
		if let Some(Quality3::Voiceless) = self.4 {
			return true;
		} else {
			return false;
		}
	}

	/// Get radical's alternative.
	fn alternative(self) -> Option<&'static str> {
		if let Some(Quality3::Alternative(alternative)) = self.4 {
			return Some(alternative);
		} else {
			return None;
		}
	}

	/// Get if radical has a quality that removes it a the beginning of a word.
	fn is_beginning(self) -> bool {
		if let Some(Quality4::Beginning) = self.5 {
			return true;
		} else {
			return false;
		}
	}

	/// Get if radical has a quality that removes it a the end of a word.
	fn is_end(self) -> bool {
		if let Some(Quality4::End) = self.5 {
			return true;
		} else {
			return false;
		}
	}
}

/// Quality of radical.
#[derive(Clone, Copy, Debug)]
enum Quality1 {
	/// Neutral radical.
	Neutral,
	/// Dark radical.
	Dark,
	/// Light radical.
	Light,
}

impl Quality1 {
	/// Get's `str` form of quality.
	fn as_str(self) -> &'static str {
		return match self {
			Self::Neutral => "",
			Self::Dark => "ˤ",
			Self::Light => "ʲ",
		};
	}
}

/// Shading and rounding.
#[derive(Clone, Copy, Debug)]
enum Quality2 {
	/// Shading.
	Shading,
	/// Rounding.
	Rounding,
}

/// Voiceless and alternatives.
#[derive(Clone, Copy, Debug)]
enum Quality3 {
	/// Voiceless.
	Voiceless,
	/// Alternative consonant.
	Alternative(&'static str),
}

/// Remove if at beginning or end.
#[derive(Clone, Copy, Debug)]
enum Quality4 {
	/// Remove when its at the beginning.
	Beginning,
	/// Remove when its at the End.
	End,
}

/// Stores vowels.
/// Purely there to make it easier to extract vowels with [`Vowel`].
#[derive(Clone, Copy, Debug)]
struct Vowels(IpaVowel, IpaVowel);

impl Vowels {
	/// Gets the right vowel with [`Vowel`].
	fn get(self, vowel: Vowel) -> IpaVowel {
		return match vowel {
			Vowel::First => self.0,
			Vowel::Last => self.1,
		};
	}
}

/// Represents letter.
#[derive(Clone, Copy, Debug)]
enum IpaLetter {
	/// Radical.
	Radical(IpaRadical, bool),
	/// Vowel.
	Vowel(IpaVowel),
	/// Duplicate.
	Duplicate,
	/// Removed letter.
	Removed,
}

/// Represents vowels.
#[derive(Clone, Copy, Debug)]
enum IpaVowel {
	/// Neutral `a`.
	NeutralA,
	/// Neutral `e`.
	NeutralE,
	/// Neutral `i`.
	NeutralI,
	/// Neutral `u`.
	NeutralU,
	/// Dark `a`.
	DarkA,
	/// Dark `u`.
	DarkU,
	/// Light `e`.
	LightE,
	/// Dark `i`.
	LightI,
	/// Nasal `a`.
	NasalA,
	/// Nasal `e`.
	NasalE,
	/// Nasal `i`.
	NasalI,
	/// Nasal `u`.
	NasalU,
	/// Shading `e`.
	ShadingE,
	/// Shading `i`.
	ShadingI,
	/// Rounding `a`.
	RoundingA,
	/// Rounding `e`.
	RoundingE,
	/// Rounding `i`.
	RoundingI,
}

impl IpaVowel {
	/// Get in `str` form.
	fn as_str(self) -> &'static str {
		return match self {
			Self::NeutralA | Self::DarkA => "a",
			Self::NeutralE | Self::LightE => "e",
			Self::NeutralI | Self::LightI => "i",
			Self::NeutralU | Self::DarkU => "u",
			Self::NasalA => "ɑ̃",
			Self::NasalE => "ɔ̃",
			Self::NasalI => "ɛ̃",
			Self::NasalU => "œ̃",
			Self::ShadingE => "æ",
			Self::ShadingI => "ɨ",
			Self::RoundingA => "ɔ",
			Self::RoundingE => "ø",
			Self::RoundingI => "y",
		};
	}

	/// Turn vowel dark.
	fn as_dark(&mut self) {
		match self {
			Self::NeutralE => *self = Self::DarkA,
			Self::NeutralI => *self = Self::DarkU,
			_ => (),
		}
	}

	/// Turn vowel light.
	fn as_light(&mut self) {
		match self {
			Self::NeutralA | Self::DarkA => *self = Self::LightE,
			Self::NeutralU | Self::DarkU => *self = Self::LightI,
			_ => (),
		}
	}

	/// Turn vowel nasal.
	fn as_nasal(&mut self) {
		match self {
			Self::NeutralA | Self::DarkA => *self = Self::NasalA,
			Self::NeutralE | Self::LightE => *self = Self::NasalE,
			Self::NeutralI | Self::LightI => *self = Self::NasalI,
			Self::NeutralU | Self::DarkU => *self = Self::NasalU,
			_ => (),
		}
	}

	/// Turn vowel shading.
	fn as_shading(&mut self) {
		match self {
			Self::LightE => *self = Self::ShadingE,
			Self::LightI => *self = Self::ShadingI,
			_ => (),
		}
	}

	/// Turn vowel rounding.
	fn as_rounding(&mut self) {
		match self {
			Self::NeutralA | Self::DarkA => *self = Self::RoundingA,
			Self::NeutralE | Self::LightE => *self = Self::RoundingE,
			Self::NeutralI | Self::LightI => *self = Self::RoundingI,
			_ => (),
		}
	}
}
