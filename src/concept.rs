use alloc::{
	slice::SliceConcatExt,
	string::{String, ToString},
	vec::Vec
};
use unchecked_unwrap::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
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

impl Concept {
	pub const CONSONANTS: [&'static str; 44] = [
		"?", "Y", "w", "h", "2", "H", "K", "k", "X", "x", "8", "4", "G", "g", "j", "7", "3", "Q", "c", "9", "S", "s", "Z", "z", "D", "d", "T", "t",
		"P", "0", "B", "6", "V", "f", "p", "b", "m", "n", "O", "1", "R", "r", "L", "l",
	];

	const FORMS: [(Option<usize>, Option<usize>); 8] = [
		(None, None),
		(None, Some(3)),
		(None, Some(2)),
		(None, Some(5)),
		(None, Some(6)),
		(Some(1), Some(4)),
		(Some(1), Some(5)),
		(Some(1), Some(6))
	];

	const STEMS: [(&'static str, &'static str); 4] = [("o", "o"), ("o", "ı"), ("ı", "o"), ("ı", "ı")];

	// increments index letter and resets letters with lower index and makes sure all indexed letters don't have collisions
	fn increment_index(&mut self, index: usize) {
		debug_assert!(index < self.concept.len());

		// assert checked up there
		let letter = unsafe { self.concept.get_unchecked_mut(index) };

		// increment selected letter
		*letter += 1;

		// if we reached maximum character set to 0
		if letter == &Self::CONSONANTS.len() {
			*letter = 0;
		}

		// reset all lower index letters
		for (index, letter) in self.concept.iter_mut().enumerate().skip(index + 1) {
			// get default letter for position
			// can not fail because we are looping through the same kind of structure
			let default_concept = unsafe { *Self::default().concept.get_unchecked(index) };
			*letter = default_concept;
		}

		// make sure we don't end up with collisions
		// we loop through all letters with the index given and lower
		'outer: for (index, outer_letter) in self.concept.iter().enumerate().skip(index) {
			// we loop through all letters higher then the current index
			for inner_letter in self.concept.iter().rev().skip(self.concept.len() - index) {
				// check if we found a collision
				if inner_letter == outer_letter {
					// increment letter we found collision on
					self.increment_index(index);
					// stop the whole thing because we go through it anyway now
					// otherwise we would end up with an almost endless loop
					break 'outer;
				}
			}
		}
	}

	// little convenience function
	fn decrement_index(mut index: usize) -> usize {
		if index == 0 {
			index = Self::CONSONANTS.len();
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
		debug_assert!(index < (Self::CONSONANTS.len()) * (Self::CONSONANTS.len() - 1) * (Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3));

		// reset concept
		let mut concept = Self::default();

		// calculate amount of pages
		let pages = index / ((Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3));

		// loop through pages - but skip page 0 so we can do odd/even calculations properly
		for page in 1..=pages {
			// increment 1st letter every time we finish through the 2nd letter
			// which is full length - 1
			if page % (Self::CONSONANTS.len() - 1) == 0 {
				concept.increment_index(0);
			}
			// otherwise increment second letter
			else {
				concept.increment_index(1);
			}
		}

		// then get the rest of the indexes and increment them concept by concept
		for _ in pages * ((Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3))..index {
			concept.increment_concept();
		}

		return concept;
	}

	// converts concept to string
	pub fn to_string(&self) -> String {
		// check out of bounds
		debug_assert!(self.concept[0] < Self::CONSONANTS.len());
		debug_assert!(self.concept[1] < Self::CONSONANTS.len());
		debug_assert!(self.concept[2] < Self::CONSONANTS.len());
		debug_assert!(self.concept[3] < Self::CONSONANTS.len());

		return format!(
			"{}{}{}{}",
			// assert checked all indexes up there
			unsafe { Self::CONSONANTS.get_unchecked(self.concept[1]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.concept[3]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.concept[2]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.concept[0]) }
		);
	}

	// generate the four forms to a stem
	fn generate_stem(&self, prefix: &str, suffix: &str, l_infix: Option<usize>, duplicate: Option<usize>) -> String {
		// save default concept
		let mut form = self.to_string();
		// do some sanity checks
		debug_assert!(2 + if let Some(l_infix) = l_infix { l_infix } else { 0 } <= form.grapheme_len());

		// insert first vocal
		form.grapheme_insert(1, prefix);
		// insert second vocal - suffix is the amount of consonants from behind
		form.grapheme_insert(form.grapheme_len() - 1, suffix);

		// if there is a third vocal insert that too
		if let Some(l_infix) = l_infix {
			// l_infix is the amount of consonants between it and the first vocal
			// so don't forget + 1 because we added the first vocal before already
			// the third vocal uses the same character as the first one
			form.grapheme_insert(1 + l_infix + 1, prefix);
		}

		// add the duplicate
		if let Some(duplicate) = duplicate {
			// sanity checks
			debug_assert!(duplicate <= form.grapheme_len() + if l_infix.is_some() { 3 } else { 2 });

			// the duplicate is the same character as the last character before it in the concept
			// we have an assert above
			let duplicate_letter = unsafe { form.grapheme_nth(duplicate - 1).unchecked_unwrap().to_string() };
			form.grapheme_insert(duplicate, &duplicate_letter);
		}

		return form;
	}

	fn generate_stems(&self, l_infix: Option<usize>, duplicate: Option<usize>) -> [String; 4] {
		let mut stems: [String; 4] = Default::default();

		for ((prefix, suffix), ref mut stem) in Self::STEMS.iter().zip(stems.iter_mut()) {
			*stem = &mut self.generate_stem(prefix, suffix, l_infix, duplicate);
		}

		return stems;
	}

	// generate all forms within a concept and return them
	pub fn generate_forms(&self) -> [[String; 4]; 8] {
		let mut forms: [[String; 4]; 8] = Default::default();

		for ((l_infix, duplicate), ref mut form) in Self::FORMS.iter().zip(forms.iter_mut()) {
			*form = &mut self.generate_stems(*l_infix, *duplicate);
		}

		return forms;
	}

	// build concept from string
	pub fn from_string(string: &str) -> Result<Self, &str> {
		// sanity check
		debug_assert!(string.grapheme_len() == Self::default().concept.len());

		// insert letters
		// we did assert above, no run-time checks needed - except for invalid letters
		let letters = [
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(0).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(1).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(2).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.grapheme_nth(3).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?
		];

		// check for duplicate keys
		// looping through all letters
		for (key_outer, letter_outer) in letters.iter().enumerate() {
			// looping through all letters again to find a duplicate
			for (key_inner, letter_inner) in letters.iter().enumerate() {
				// make sure we are not comparing the same letter to each other
				if key_outer != key_inner && letter_outer == letter_inner {
					// if we found a duplicate return an error
					return Err("Duplicate letters found.");
				}
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
		let mut multiplier = Self::CONSONANTS.len() * (Self::CONSONANTS.len() - 1) * (Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3);

		for (index, letter) in self.concept.iter().enumerate() {
			// multiplier is reduced on every iteration
			multiplier /= Self::CONSONANTS.len() - index;

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
