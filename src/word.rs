use alloc::string::String;
use unchecked_unwrap::*;

#[derive(PartialEq)]
pub struct Word {
	word: [usize; 4]
}

impl Default for Word {
	fn default() -> Self {
		return Self { word: [3, 0, 2, 1] };
	}
}

impl Word {
	pub const CONSONANTS: [char; 44] = [
		'?', 'Y', 'w', 'h', '2', 'H', 'K', 'k', 'X', 'x', '8', '4', 'G', 'g', 'j', '7', '3', 'Q', 'c', '9', 'S', 's', 'Z', 'z', 'D', 'd', 'T', 't',
		'P', '0', 'B', '6', 'V', 'f', 'p', 'b', 'm', 'n', 'o', '1', 'R', 'r', 'L', 'l',
	];

	// increments index letter and resets letters with lower index and makes sure all indexed letters dont have collisions
	fn increment_index(&mut self, index: usize) {
		debug_assert!(index < self.word.len());

		// assert checked up there
		let letter = unsafe { self.word.get_unchecked_mut(index) };

		// increment selected letter
		*letter += 1;

		// if we reached maximum character set to 0
		if letter == &Self::CONSONANTS.len() {
			*letter = 0;
		}

		// reset all lower index letters
		for (index, letter) in self.word.iter_mut().enumerate().skip(index + 1) {
			// get default letter for position
			// can not fail because we are looping through the same kind of structure
			let default_word = unsafe { *Self::default().word.get_unchecked(index) };
			*letter = default_word;
		}

		// make sure we don't end up with collisions
		// we loop through all letters with the index given and lower
		'outer: for (index, outer_letter) in self.word.iter().enumerate().skip(index) {
			// we loop through all letters higher then the current index
			for inner_letter in self.word.iter().rev().skip(self.word.len() - index) {
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
			index = Word::CONSONANTS.len();
		}

		return index - 1;
	}

	// functio to calculate character limit every letter can reach
	// checks for collisions are built-in
	fn get_limit(&self, index: usize) -> usize {
		// sanity check
		debug_assert!(index < self.word.len());

		// the closest limit is one character before the default
		let mut limit = Self::decrement_index(unsafe { *Self::default().word.get_unchecked(index) });

		// then we have to check for collisions
		'outer_loop: loop {
			// we only want to check letters that are higher then the index given
			for letter in self.word.iter().rev().skip(self.word.len() - index) {
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

	// increment by one word
	pub fn increment_word(&mut self) {
		// we want to search which letter reached its limit first - backwards
		for (index, letter) in self.word.iter().enumerate().rev() {
			// check if it reached its limit
			// but the last letter doesn't have any limit that we are interested in here
			if letter == &self.get_limit(index) || index == 0 {
				// if it did, we need to go higher up!
				continue;
			}

			// if we found the first letter then doesn't have a limit, increase it by one
			self.increment_index(index);
			// and break!
			break;
		}
	}

	// set to arbitrary position in the list
	pub fn from_word_index(index: usize) -> Self {
		// reset word
		let mut word = Self::default();

		// calculate amount of pages
		let pages = index / ((Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3));

		// loop through pages - but skip page 0 so we can do odd/even calculations properly
		for page in 1..=pages {
			// increment 1st letter every time we finish through the 2nd letter
			// which is full length - 1
			if page % (Self::CONSONANTS.len() - 1) == 0 {
				word.increment_index(0);
			}
			// otherwise increment second letter
			else {
				word.increment_index(1);
			}
		}

		// then get the rest of the indexes and increment them word by word
		for _ in pages * ((Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3))..index {
			word.increment_word();
		}

		return word;
	}

	// converts word to string
	pub fn to_string(&self) -> String {
		// check out of bounds
		debug_assert!(self.word[0] < Self::CONSONANTS.len());
		debug_assert!(self.word[1] < Self::CONSONANTS.len());
		debug_assert!(self.word[2] < Self::CONSONANTS.len());
		debug_assert!(self.word[3] < Self::CONSONANTS.len());

		return format!(
			"{}{}{}{}",
			// assert checked all indexes up there
			unsafe { Self::CONSONANTS.get_unchecked(self.word[1]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.word[3]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.word[2]) },
			unsafe { Self::CONSONANTS.get_unchecked(self.word[0]) }
		);
	}

	// generate the four forms to a stem
	fn generate_stem(&self, prefix: usize, suffix: usize, l_infix: Option<usize>, duplicate: Option<usize>) -> [String; 4] {
		let mut forms: [String; 4] = Default::default();
		let word = self.to_string();

		// loop through the four possible forms
		for (iter, vocals) in [('v', 'v'), ('v', 'y'), ('y', 'v'), ('y', 'y')].iter().enumerate() {
			// we know that we only have four forms
			let form = unsafe { forms.get_unchecked_mut(iter) };
			// save default word
			*form = word.clone();
			// do some sanity checks
			debug_assert!(prefix + suffix + if let Some(l_infix) = l_infix { l_infix } else { 0 } <= form.len());

			// insert first vocal
			form.insert(prefix, vocals.0);
			// insert second vocal - suffix is the amount of consonants from behind
			form.insert(form.len() - suffix, vocals.1);

			// if there is a third vocal insert that too
			if let Some(l_infix) = l_infix {
				// l_infix is the amount of consonants between it and the first vocal
				// so don't forget + 1 because we added the first vocal before already
				// the third vocal uses the same character as the first one
				form.insert(prefix + l_infix + 1, vocals.0);
			}

			// add the duplicate
			if let Some(duplicate) = duplicate {
				// sanity checks
				debug_assert!(duplicate <= form.len() + if l_infix.is_some() { 3 } else { 2 });

				// the duplicate is the same character as the last character before it in the word
				// we have an assert above
				let duplicate_letter = unsafe { form.chars().nth(duplicate - 1).unchecked_unwrap() };
				form.insert(duplicate, duplicate_letter);
			}
		}

		return forms;
	}

	// generate all forms within a word and return them
	pub fn generate_forms(&self) -> [[String; 4]; 8] {
		let mut strings: [[String; 4]; 8] = Default::default();

		strings[0] = self.generate_stem(1, 1, None, None);
		strings[1] = self.generate_stem(1, 1, None, Some(3));
		strings[2] = self.generate_stem(1, 1, None, Some(2));
		strings[3] = self.generate_stem(1, 1, None, Some(5));
		strings[4] = self.generate_stem(1, 1, None, Some(6));
		strings[5] = self.generate_stem(1, 1, Some(1), Some(4));
		strings[6] = self.generate_stem(1, 1, Some(1), Some(5));
		strings[7] = self.generate_stem(1, 1, Some(1), Some(6));

		return strings;
	}

	// build word from string
	pub fn from_string(string: &str) -> Result<Self, &str> {
		// sanity check
		debug_assert!(string.len() == Self::default().word.len());

		// insert letters
		// we did assert above, no run-time checks needed - except for invalid letters
		let letters = [
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.chars().nth(0).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.chars().nth(1).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.chars().nth(2).unchecked_unwrap() };
				})
				.ok_or("Invalid letters.")?,
			Self::CONSONANTS
				.iter()
				.position(|letter| {
					return letter == unsafe { &string.chars().nth(3).unchecked_unwrap() };
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

		// build word and return
		// make sure we are using the right order for things
		// this may seem counter-intuitive at first, but letter positions are symmetrical to storage positions, not equal!
		return Ok(Self {
			word: [letters[3], letters[0], letters[2], letters[1]]
		});
	}

	// get word from word index
	pub fn get_word_index(&self) -> usize {
		let mut search_word = Self::default();
		let mut word_index = 0;
		let mut multiplier = Self::CONSONANTS.len() * (Self::CONSONANTS.len() - 1) * (Self::CONSONANTS.len() - 2) * (Self::CONSONANTS.len() - 3);

		for index in 0..search_word.word.len() {
			let letter = *unsafe { self.word.get_unchecked(index) };
			multiplier /= Self::CONSONANTS.len() - index;

			loop {
				if unsafe { search_word.word.get_unchecked(index) } == &letter {
					break;
				}
				else {
					search_word.increment_index(index);
					word_index += multiplier;
				}
			}
		}

		return word_index;
	}
}
