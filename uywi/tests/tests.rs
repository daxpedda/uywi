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

#[cfg(test)]
mod tests {
	use rayon::prelude::*;
	use std::str::FromStr;
	use uywi::*;

	#[test]
	fn touch_all() {
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
							concept == Concept::from_str(&concept.to_string()).expect("failed to create concept"),
							"concept index is wrong"
						);

						for stem in concept {
							for form in stem {
								form.to_string();
							}
						}

						index += 1;
					}
				});
			});
		});
	}
}
