#![warn(clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::indexing_slicing,
	clippy::needless_return,
	clippy::non_ascii_literal,
	clippy::missing_inline_in_public_items,
	clippy::option_expect_used,
	clippy::panic,
	clippy::result_expect_used,
	clippy::shadow_reuse,
	clippy::shadow_same
)]

use rayon::prelude::*;
use uywi::{Concept, Length, Pages, Script};

#[test]
fn touch_all() {
	let script = Script::UywiChiffre;

	[Length::L2, Length::L3, Length::L4].into_par_iter().for_each(|length| {
		Pages::new(*length).par_bridge().for_each(|page| {
			page.into_iter().par_bridge().for_each(|row| {
				let mut index = page.index() * length.concepts_per_page() + row.index() * length.concepts_per_row();

				for concept in row {
					assert!(index == concept.index(), "concept index is wrong");
					assert!(
						concept == Concept::from_index(concept.index(), *length).expect("failed to create concept"),
						"concept index is wrong"
					);
					assert!(
						concept == script.from_concept(&concept.to_string(script)).expect("failed to create concept"),
						"concept index is wrong"
					);

					for stem in concept {
						for form in stem {
							let _word = form.to_string(script);
						}
					}

					index += 1;
				}
			});
		});
	});
}
