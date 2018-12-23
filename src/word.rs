use alloc::string::String;
use core::hint::unreachable_unchecked;
use unchecked_unwrap::*;

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
	pub const CONSONANTS: [char; 44] = [
		'?', 'Y', 'w', 'h', '2', 'H', 'K', 'k', 'X', 'x', '8', '4', 'G', 'g', 'j', '7', '3', 'Q', 'c', '9', 'S', 's', 'Z', 'z', 'D', 'd', 'T', 't',
		'P', '0', 'B', '6', 'V', 'f', 'p', 'b', 'm', 'n', 'o', '1', 'R', 'r', 'L', 'l',
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

	pub fn to_string(&self) -> String {
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

	fn generate_form(&self, prefix: usize, suffix: usize, r_infix: Option<usize>, duplicate: Option<usize>) -> [String; 4] {
		let mut forms: [String; 4] = Default::default();

		for (iter, vocals) in [('v', 'v'), ('v', 'y'), ('y', 'v'), ('y', 'y')].iter().enumerate() {
			let form = unsafe { forms.get_unchecked_mut(iter) };
			*form = self.to_string();
			let word_len = form.len();
			let mut vocal_num = 2;

			debug_assert!(prefix + suffix <= word_len);
			form.insert(prefix, vocals.0);
			form.insert(form.len() - suffix, vocals.1);

			if let Some(r_infix) = r_infix {
				debug_assert!(prefix + suffix + r_infix <= word_len);
				vocal_num = 3;
				form.insert(prefix + r_infix + 1, vocals.0);
			}

			if let Some(duplicate) = duplicate {
				debug_assert!(duplicate <= word_len + vocal_num);

				unsafe {
					let duplicate_letter = form.get_unchecked(duplicate - 1..duplicate).chars().nth(0).unchecked_unwrap();
					form.insert(duplicate, duplicate_letter);
				}
			}
		}

		return forms;
	}

	pub fn generate_forms(&self) -> [[String; 4]; 8] {
		let mut strings: [[String; 4]; 8] = Default::default();

		strings[0] = self.generate_form(1, 1, None, None);
		strings[1] = self.generate_form(1, 1, None, Some(3));
		strings[2] = self.generate_form(1, 1, None, Some(2));
		strings[3] = self.generate_form(1, 1, None, Some(5));
		strings[4] = self.generate_form(1, 1, None, Some(6));
		strings[5] = self.generate_form(1, 1, Some(1), Some(4));
		strings[6] = self.generate_form(1, 1, Some(1), Some(5));
		strings[7] = self.generate_form(1, 1, Some(1), Some(6));

		return strings;
	}

	pub fn get_id(&self) -> [usize; 4] {
		return self.word;
	}

	pub fn new(id: [usize; 4]) -> Self {
		return Self { word: id };
	}
}
