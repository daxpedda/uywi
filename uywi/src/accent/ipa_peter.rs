//! IPA (Peter's accent)

use super::*;
use crate::*;
use arrayvec::{ArrayString, ArrayVec};

/// Accent instantiation. Used to return from enum without [`Box`].
pub(crate) const IPA_PETER: IpaPeter = IpaPeter {};

/// UYWI Chiffre.
#[derive(Clone, Copy, Debug)]
pub(crate) struct IpaPeter {}

impl AccentExt for IpaPeter {
	fn build_concept(&self, _string: &str) -> Result<Concept> {
		unimplemented!("build concept for ipa peter is unimplemented")
	}

	fn build_concept_string(&self, concept: Concept) -> ArrayString<[u8; 64]> {
		let mut string = ArrayString::new();

		for radical in concept.radicals() {
			let radical = accent_radicals()[usize::from(radical.index())];
			string.push_str(radical.radical());
			string.push_str(radical.quality().as_str());
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

		let mut letters = ArrayVec::<[ArrayString<[_; 8]>; 9]>::new();

		for (position, letter_structure) in structure.into_iter().enumerate() {
			let mut letter = ArrayString::<[_; 8]>::new();

			// first level
			match letter_structure {
				Letter::Consonant(radical_index) => {
					let concept_radical_index = usize::from(*radical_index);
					let radical_index = usize::from(concept_radicals[concept_radical_index].index());
					let radical = accent_radicals()[radical_index];

					letter.push_str(radical.radical());

					if radical.quality_visible() {
						letter.push_str(radical.quality().as_str());
					}
				},
				Letter::Vocal(vocal) => letter.push_str(vocals.get(*vocal)),
				Letter::Duplicate => letter.push_str(&letters[position.psub(1)]),
			};

			letters.push(letter);
		}

		let mut word = ArrayString::new();

		for letter in letters {
			word.push_str(letter.as_str())
		}

		return word;
	}
}

/// List of radicals with all exceptions.
const fn accent_radicals() -> [IpaRadical; NUM_OF_RADICALS] {
	use Assimilation2::*;
	use Assimilation3::*;
	use Assimilation4::*;
	use Assimilation5::*;
	use Quality::*;

	#[rustfmt::skip]
	return [
        IpaRadical("ʔ",  Neutral, false, None,           None,             Some(Exception2), None),
        IpaRadical("j",  Dark,    false, None,           None,             None,             None),
        IpaRadical("w",  Neutral, false, None,           None,             None,             None),
        IpaRadical("h",  Neutral, false, None,           Some(Voiceless),  Some(Exception3), None),
        IpaRadical("ʕ",  Neutral, false, None,           None,             None,             None),
        IpaRadical("ħ",  Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("k",  Dark,    false, Some(Shading),  Some(Voiceless),  None,             None),
        IpaRadical("kʰ", Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("x",  Dark,    false, Some(Shading),  Some(Voiceless),  None,             None),
        IpaRadical("x",  Light,   false, None,           Some(Voiceless),  None,             None),
        IpaRadical("ʁ",  Neutral, false, None,           Some(Exception1), None,             None),
        IpaRadical("ɟ",  Neutral, false, None,           Some(Exception1), None,             None),
        IpaRadical("g",  Dark,    false, Some(Shading),  None,             None,             None),
        IpaRadical("g",  Light,   false, None,           None,             None,             None),
        IpaRadical("ɥ",  Light,   false, Some(Rounding), None,             None,             None),
        IpaRadical("d͡ʐ", Dark,    false, Some(Shading),  None,             None,             None),
        IpaRadical("d͡ʒ", Neutral, false, None,           None,             None,             None),
        IpaRadical("ʂ",  Dark,    false, Some(Shading),  Some(Voiceless),  None,             None),
        IpaRadical("ɕ",  Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("ʃ",  Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("s",  Dark,    false, Some(Shading),  Some(Voiceless),  None,             None),
        IpaRadical("s",  Light,   true,  None,           Some(Voiceless),  None,             None),
        IpaRadical("z",  Dark,    false, Some(Shading),  None,             None,             None),
        IpaRadical("z",  Light,   false, None,           None,             None,             Some(Palatalization)),
        IpaRadical("d",  Dark,    false, Some(Shading),  None,             None,             None),
        IpaRadical("d",  Light,   false, None,           None,             None,             Some(Palatalization)),
        IpaRadical("t",  Dark,    false, Some(Shading),  Some(Voiceless),  None,             None),
        IpaRadical("tʰ", Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("t͡ɕ", Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("t͡s", Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("θ",  Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("ð",  Neutral, false, None,           None,             None,             None),
        IpaRadical("v",  Dark,    false, None,           None,             None,             None),
        IpaRadical("f",  Neutral, false, None,           Some(Voiceless),  None,             None),
        IpaRadical("p",  Neutral, false, Some(Rounding), Some(Voiceless),  None,             None),
        IpaRadical("b",  Neutral, false, Some(Rounding), None,             None,             None),
        IpaRadical("m",  Neutral, false, Some(Rounding), None,             None,             None),
        IpaRadical("n",  Neutral, false, None,           None,             None,             None),
        IpaRadical("ŋ",  Neutral, false, None,           None,             None,             None),
        IpaRadical("ɻ",  Dark,    false, Some(Rounding), None,             None,             None),
        IpaRadical("r",  Neutral, false, None,           None,             None,             None),
        IpaRadical("l",  Light,   true,  None,           None,             None,             None),
        IpaRadical("ʟ",  Dark,    false, Some(Shading),  None,             None,             None),
        IpaRadical("l",  Neutral, false, None,           None,             None,             None),
	];
}

/// List how forms are configured.
fn form_configs(length: Length, form_index: u8) -> Vocals {
	let mut configs = ArrayVec::<[_; 4]>::new();

	match length {
		Length::L2 => configs.try_extend_from_slice(&[Vocals("a", "a"), Vocals("i", "i")]),
		Length::L3 | Length::L4 => configs.try_extend_from_slice(&[Vocals("a", "e"), Vocals("e", "i"), Vocals("u", "a"), Vocals("u", "i")]),
	}
	.expect("failed to fill form configs");

	return configs[usize::from(form_index)];
}

/// Save radicals with all the exceptions.
#[derive(Clone, Copy, Debug)]
struct IpaRadical(
	&'static str,
	Quality,
	bool,
	Option<Assimilation2>,
	Option<Assimilation3>,
	Option<Assimilation4>,
	Option<Assimilation5>,
);

impl IpaRadical {
	/// Get radical in string form.
	const fn radical(self) -> &'static str {
		return self.0;
	}

	/// Get radical's quality.
	const fn quality(self) -> Quality {
		return self.1;
	}

	/// Get radical's quality.
	const fn quality_visible(self) -> bool {
		return self.2;
	}
}

/// First level of assmiliation.
#[derive(Clone, Copy, Debug)]
enum Quality {
	/// Neutral radical.
	Neutral,
	/// Dark radical.
	Dark,
	/// Light radical.
	Light,
}

impl Quality {
	/// Get's `str` form of quality.
	fn as_str(self) -> &'static str {
		return match self {
			Self::Neutral => "",
			Self::Dark => "ˤ",
			Self::Light => "ʲ",
		};
	}
}

///
#[derive(Clone, Copy, Debug)]
enum Assimilation2 {
	/// ^
	Shading,
	/// ʷ
	Rounding,
}

///
#[derive(Clone, Copy, Debug)]
enum Assimilation3 {
	/// '
	Voiceless,
	/// 8 & 4
	Exception1,
}

///
#[derive(Clone, Copy, Debug)]
enum Assimilation4 {
	/// ?
	Exception2,
	/// h
	Exception3,
}

///
#[derive(Clone, Copy, Debug)]
enum Assimilation5 {
	///
	Palatalization,
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
