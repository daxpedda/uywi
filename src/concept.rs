use alloc::{
	format,
	slice::SliceConcatExt,
	string::{String, ToString},
	vec::Vec
};
use unchecked_unwrap::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Clone)]
pub struct Concept {
	concept: [usize; 4]
}

impl Default for Concept {
	fn default() -> Self {
		return Self { concept: [3, 0, 2, 1] };
	}
}

trait GraphemeStr {
	fn grapheme_len(&self) -> usize;

	fn grapheme_nth(&'_ self, n: usize) -> Option<&'_ str>;
}

impl GraphemeStr for str {
	fn grapheme_len(&self) -> usize {
		return (self.graphemes(true).collect(): Vec<&Self>).len();
	}

	fn grapheme_nth(&'_ self, n: usize) -> Option<&'_ str> {
		return self.graphemes(true).nth(n);
	}
}

impl GraphemeStr for String {
	fn grapheme_len(&self) -> usize {
		return (self.graphemes(true).collect(): Vec<&str>).len();
	}

	fn grapheme_nth(&'_ self, n: usize) -> Option<&'_ str> {
		return self.graphemes(true).nth(n);
	}
}

trait GraphemeString {
	fn grapheme_insert(&mut self, idx: usize, string: &str);
}

impl GraphemeString for String {
	fn grapheme_insert(&mut self, idx: usize, string: &str) {
		let mut vec = self.graphemes(true).collect(): Vec<&str>;
		vec.insert(idx, string);
		*self = vec.concat();
	}
}

type VocalsPair = (&'static str, &'static str);

impl Concept {
	pub const RADICALS: [&'static str; 44] = [
		"?", "Y", "w", "h", "2", "H", "K", "k", "X", "x", "8", "4", "G", "g", "j", "7", "3", "Q", "c", "9", "S", "s", "Z", "z", "D", "d", "T", "t",
		"P", "0", "B", "6", "V", "f", "p", "b", "m", "n", "O", "1", "R", "r", "L", "l",
	];

	pub const STEMS: [(Option<usize>, Option<usize>, bool); 8] = [
		(None, None, true),
		(None, Some(3), true),
		(None, Some(2), true),
		(None, Some(5), true),
		(None, Some(6), true),
		(Some(1), Some(4), false),
		(Some(1), Some(5), false),
		(Some(1), Some(6), true)
	];

	pub const FORMS: [VocalsPair; 4] = [("o", "o"), ("o", "ı"), ("ı", "o"), ("ı", "ı")];

	const INTONATIONS_MAP: VocalsPair = ("o", "ı");

	const INTONATIONS: [(VocalsPair, VocalsPair); 9] = [
		(("ò", "ì"), ("ò", "ì")),
		(("ò", "ì"), ("ó", "í")),
		(("ó", "í"), ("ò", "ì")),
		(("ò", "ì"), ("ō", "ī")),
		(("ō", "ī"), ("ò", "ì")),
		(("ó", "í"), ("ó", "í")),
		(("ó", "í"), ("ō", "ī")),
		(("ō", "ī"), ("ó", "í")),
		(("ō", "ī"), ("ō", "ī"))
	];

	// increments index letter and resets letters with lower index and makes sure all indexed letters don't have collisions
	fn increment_index(&mut self, index: usize) {
		debug_assert!(index < self.concept.len());

		// assert checked up there
		let letter = unsafe { self.concept.get_unchecked_mut(index) };

		// increment selected letter
		*letter += 1;

		// if we reached maximum character set to 0
		if letter == &Self::RADICALS.len() {
			*letter = 0;
		}

		// reset all lower index letters
		for (letter, reset_letter) in self
			.concept
			.iter_mut()
			.skip(index + 1)
			// get default letter for position
			.zip(Self::default().concept.iter().skip(index + 1))
		{
			*letter = *reset_letter;
		}

		// make sure we don't end up with collisions
		// we loop through all letters with the index given and lower
		'outer: for (index, outer_letter) in self.concept.iter().enumerate().skip(index) {
			// we loop through all letters higher then the current index
			if self
				.concept
				.iter()
				.enumerate()
				.filter(|(inner_index, _)| return *inner_index < index)
				.rev()
				// check if we found a collision
				.any(|(_, inner_letter)| return inner_letter == outer_letter)
			{
				// increment letter we found collision on
				self.increment_index(index);
				// stop the whole thing because we go through it anyway now
				// otherwise we would end up with an almost endless loop
				break 'outer;
			}
		}
	}

	// little convenience function
	fn decrement_index(mut index: usize) -> usize {
		if index == 0 {
			index = Self::RADICALS.len();
		}

		return index - 1;
	}

	// function to calculate character limit every letter can reach
	// checks for collisions are built-in
	fn get_limit(&self, index: usize) -> usize {
		// sanity check
		debug_assert!(index < self.concept.len());

		// the closest limit is one character before the default
		let mut limit = Self::decrement_index(unsafe { *Self::default().concept.get_unchecked(index) });

		// then we have to check for collisions
		'outer_loop: loop {
			// we only want to check letters that are higher then the index given
			for letter in self.concept.iter().rev().skip(self.concept.len() - index) {
				// check if we found a collision
				if &limit == letter {
					// decrease limit by one step
					limit = Self::decrement_index(limit);
					// and start checking from scratch again
					continue 'outer_loop;
				}
			}

			// if we found no collision - break
			break;
		}

		return limit;
	}

	// increment by one concept
	pub fn increment_concept(&mut self) {
		// we want to search which letter reached its limit first - backwards
		for (index, letter) in self.concept.iter().enumerate().rev() {
			// check if it reached its limit
			// but the last letter doesn't have any limit that we are interested in here
			if letter == &self.get_limit(index) || index == 0 {
				// if it did, we need to go higher up!
				continue;
			}

			// if we found the first letter, then there isn't a limit, increase it by one
			self.increment_index(index);
			// and break!
			break;
		}
	}

	// set to arbitrary position in the list
	pub fn from_concept_index(index: usize) -> Self {
		debug_assert!(index < (Self::RADICALS.len()) * (Self::RADICALS.len() - 1) * (Self::RADICALS.len() - 2) * (Self::RADICALS.len() - 3));

		// reset concept
		let mut concept = Self::default();

		// calculate amount of pages
		let pages = index / ((Self::RADICALS.len() - 2) * (Self::RADICALS.len() - 3));

		// loop through pages - but skip page 0 so we can do odd/even calculations properly
		for page in 1..=pages {
			// increment 1st letter every time we finish through the 2nd letter
			// which is full length - 1
			if page % (Self::RADICALS.len() - 1) == 0 {
				concept.increment_index(0);
			}
			// otherwise increment second letter
			else {
				concept.increment_index(1);
			}
		}

		// then get the rest of the indexes and increment them concept by concept
		for _ in pages * ((Self::RADICALS.len() - 2) * (Self::RADICALS.len() - 3))..index {
			concept.increment_concept();
		}

		return concept;
	}

	// converts concept to string
	pub fn to_string(&self) -> String {
		// check out of bounds
		debug_assert!(self.concept[0] < Self::RADICALS.len());
		debug_assert!(self.concept[1] < Self::RADICALS.len());
		debug_assert!(self.concept[2] < Self::RADICALS.len());
		debug_assert!(self.concept[3] < Self::RADICALS.len());

		return format!(
			"{}{}{}{}",
			// assert checked all indexes up there
			unsafe { Self::RADICALS.get_unchecked(self.concept[1]) },
			unsafe { Self::RADICALS.get_unchecked(self.concept[3]) },
			unsafe { Self::RADICALS.get_unchecked(self.concept[2]) },
			unsafe { Self::RADICALS.get_unchecked(self.concept[0]) }
		);
	}

	// generate a stem or intonation
	fn generate_stem_or_intonation(
		&self,
		prefix: &str,
		suffix: &str,
		l_infix: Option<usize>,
		duplicate: Option<usize>,
		intonation: Option<(&str, &str, bool)>
	) -> String {
		// unpack intonations
		let (intonation, (prefix_accented, suffix_accented, suffix_accented_bool)) = if let Some(intonation) = intonation {
			(true, intonation)
		}
		else {
			(false, ("", "", false))
		};

		// save default concept
		let mut stem = self.to_string();
		// do some sanity checks
		debug_assert!(2 + if let Some(l_infix) = l_infix { l_infix } else { 0 } <= stem.grapheme_len());

		// insert first vocal
		stem.grapheme_insert(1, if intonation { prefix_accented } else { prefix });
		// insert second vocal - suffix is the amount of consonants from behind
		stem.grapheme_insert(stem.grapheme_len() - 1, if suffix_accented_bool { suffix_accented } else { suffix });

		// if there is a third vocal insert that too
		if let Some(l_infix) = l_infix {
			// l_infix is the amount of consonants between it and the first vocal
			// so don't forget + 1 because we added the first vocal before already
			// the third vocal uses the same character as the first one
			stem.grapheme_insert(1 + l_infix + 1, if suffix_accented_bool { prefix_accented } else { prefix });
		}

		// add the duplicate
		if let Some(duplicate) = duplicate {
			// sanity checks
			debug_assert!(duplicate <= stem.grapheme_len() + if l_infix.is_some() { 3 } else { 2 });

			// the duplicate is the same character as the last character before it in the concept
			// we have an assert above
			let duplicate_letter = unsafe { stem.grapheme_nth(duplicate - 1).unchecked_unwrap().to_string() };
			stem.grapheme_insert(duplicate, &duplicate_letter);
		}

		return stem;
	}

	// generate all stems within a concept and return them
	pub fn generate_stems(&self) -> [String; 32] {
		let mut stems: [String; 32] = Default::default();

		// loop through all forms
		for (index_form, (prefix, suffix)) in Self::FORMS.iter().enumerate() {
			// loop through all stems
			for ((l_infix, duplicate, _), ref mut stem) in Self::STEMS.iter().zip(stems.iter_mut().skip(index_form * Self::STEMS.len())) {
				**stem = self.generate_stem_or_intonation(prefix, suffix, *l_infix, *duplicate, None);
			}
		}

		return stems;
	}

	// generate all nine intonations of a stem
	pub fn generate_intonations(&self, index_form: usize, index_stem: usize) -> [String; 9] {
		debug_assert!(index_form < Self::FORMS.len());
		debug_assert!(index_stem < Self::STEMS.len());

		let mut intonations: [String; 9] = Default::default();
		// sanity checks up there
		// unpacking needed details
		let (prefix, suffix) = unsafe { Self::FORMS.get_unchecked(index_form) };
		let (l_infix, duplicate, suffix_accented_bool) = unsafe { Self::STEMS.get_unchecked(index_stem) };

		// loop through all intonations
		for ((prefix_accented, suffix_accented), intonation) in Self::INTONATIONS
			.iter()
			// filtering out unneeded vocal-intonation mappings
			.map(|((prefix_0, prefix_1), (suffix_0, suffix_1))| {
				return (
					if *prefix == Self::INTONATIONS_MAP.0 { prefix_0 } else { prefix_1 },
					if *suffix == Self::INTONATIONS_MAP.0 { suffix_0 } else { suffix_1 }
				);
			})
			.zip(intonations.iter_mut())
		{
			*intonation = self.generate_stem_or_intonation(
				prefix,
				suffix,
				*l_infix,
				*duplicate,
				Some((prefix_accented, suffix_accented, *suffix_accented_bool))
			);
		}

		return intonations;
	}

	// build concept from string
	pub fn from_string(string: &str) -> Result<Self, &str> {
		// sanity check
		debug_assert!(string.grapheme_len() == Self::default().concept.len());

		// insert letters
		// we did assert above, no run-time checks needed - except for invalid letters
		let letters = [
			Self::RADICALS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(0).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::RADICALS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(1).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::RADICALS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(2).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::RADICALS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(3).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?
		];

		// check for duplicate keys
		// looping through all letters
		for (key, letter) in letters.iter().enumerate() {
			// try to find duplicate letter in the others
			// but filter same letter
			if letters
				.iter()
				.enumerate()
				.filter(|(found_key, _)| return *found_key != key)
				.any(|(_, found_letter)| return found_letter == letter)
			{
				return Err("Duplicate letters found.");
			}
		}

		// build concept and return
		// make sure we are using the right order for things
		// this may seem counter-intuitive at first, but letter positions are symmetrical to storage positions, not equal!
		return Ok(Self {
			concept: [letters[3], letters[0], letters[2], letters[1]]
		});
	}

	// get concept from concept index
	pub fn get_concept_index(&self) -> usize {
		let mut concept_index = 0;
		// we start at the beginning
		let mut search_concept = Self::default();
		// the amount of concepts we skip in every iteration
		let mut multiplier = Self::RADICALS.len() * (Self::RADICALS.len() - 1) * (Self::RADICALS.len() - 2) * (Self::RADICALS.len() - 3);

		for (index, letter) in self.concept.iter().enumerate() {
			// multiplier is reduced on every iteration
			multiplier /= Self::RADICALS.len() - index;

			// we stop the loop if we find the letter
			while unsafe { search_concept.concept.get_unchecked(index) } != letter {
				// otherwise, go to the next letter and add multiplier
				search_concept.increment_index(index);
				concept_index += multiplier;
			}
		}

		return concept_index;
	}
}
