use core::hint::unreachable_unchecked;

pub struct Word {
	word: [usize; 4]
}

impl Default for Word {
	fn default() -> Self {
		let mut word = Self { word: [0; 4] };
		word.reset_letter(0);
		word.reset_letter(1);
		word.reset_letter(2);
		word.reset_letter(3);
		return word;
	}
}

impl Word {
	pub const CONSONANTS: [char; 32] = [
	                                '?', 'y', 'w', 'h', '2', '7', 'q', '5', 'k', '8', 'g', 'j', '3', '9', 'S', 's', 'Z', 'z', 'D', 'd', 'T', 't',
	                                '0', '6', 'f', 'b', 'p', 'm', 'n', 'o', 'r', 'l',
	];

	fn reset_letter(&mut self, index: usize) {
		debug_assert!((0..self.word.len()).contains(&index));

		// assert checked up there
		*(unsafe { self.word.get_unchecked_mut(index) }) = match index {
			0 => 3,
			1 => 0,
			2 => 2,
			3 => 1,
			_ => unsafe { unreachable_unchecked() } // assert checked up there
		}
	}

	// function to fix letter compared to rest of the word
	fn fix_letter(&mut self, og_index: usize) {
		debug_assert!((0..self.word.len()).contains(&og_index));

		// fix up all letters
		for index in og_index..self.word.len() {
			// if we are fixing up the another letters then the one requested, reset it
			if index > og_index {
				self.reset_letter(index);
			}

			loop {
				// assert checked up there
				let index_letter = *(unsafe { self.word.get_unchecked(index) });

				// make sure there is not a duplicate letter
				if (index > 0 && self.word[0] == index_letter)
				   || (index > 1 && self.word[1] == index_letter)
				   || (index > 2 && self.word[2] == index_letter)
				   || (index > 3 && self.word[3] == index_letter)
				{
					// assert checked up there
					*(unsafe { self.word.get_unchecked_mut(index) }) += 1;
				}
				// go back to 0 if we reached the limit of available letters
				else if index_letter >= Self::CONSONANTS.len() {
					// assert checked up there
					*(unsafe { self.word.get_unchecked_mut(index) }) = 0;
				}
				// if everything is fine break
				else {
					break;
				}
			}
		}
	}

	// increment and fix letter
	pub fn increment_letter(&mut self, index: usize) {
		debug_assert!((0..self.word.len()).contains(&index));

		// assert checked up there
		*(unsafe { self.word.get_unchecked_mut(index) }) += 1;
		self.fix_letter(index);
	}

	pub fn to_string(&self) -> alloc::string::String {
		// check out of bounds
		debug_assert!((0..Self::CONSONANTS.len()).contains(&self.word[0]));
		debug_assert!((0..Self::CONSONANTS.len()).contains(&self.word[1]));
		debug_assert!((0..Self::CONSONANTS.len()).contains(&self.word[2]));
		debug_assert!((0..Self::CONSONANTS.len()).contains(&self.word[3]));

		return format!(
		               "{}{}{}{}",
		               // assert checked all indexes up there
		               unsafe { Self::CONSONANTS.get_unchecked(self.word[1]) },
		               unsafe { Self::CONSONANTS.get_unchecked(self.word[3]) },
		               unsafe { Self::CONSONANTS.get_unchecked(self.word[2]) },
		               unsafe { Self::CONSONANTS.get_unchecked(self.word[0]) }
		);
	}
}
