#![no_std]
#![feature(type_ascription, uniform_paths, range_contains, try_blocks, alloc)]
#![warn(
	clippy::cargo, // rls being weird
	clippy::pedantic,
	clippy::restriction
)]
#![allow(
	clippy::needless_return,
	clippy::non_ascii_literal,
	clippy::decimal_literal_representation,
	clippy::float_arithmetic,
	clippy::integer_arithmetic,	// clippy being stupid
	clippy::missing_docs_in_private_items, // rls being weird
	clippy::missing_inline_in_public_items,
	clippy::print_stdout, // rls being weird
	clippy::shadow_reuse,
	clippy::shadow_same
)]

#[macro_use]
extern crate alloc;

mod word;
use word::Word;

use alloc::boxed::Box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	//std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// if dom already loaded start script
	if document.ready_state() == "complete" {
		return onload();
	}
	// otherwise wait for dom to load
	else {
		let onload_closure = Closure::wrap(Box::new(onload) as Box<Fn() -> Result<(), JsValue>>);
		document.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
		onload_closure.forget();
	}

	return Ok(());
}

fn onload() -> Result<(), JsValue> {
	// load default page
	web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?
		.forms()
		.get_with_index(0) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("load") // get load button
		.dyn_into::<web_sys::HtmlButtonElement>()?
		.click(); // clicking on it triggers onsubmit(event)

	return Ok(());
}

#[wasm_bindgen]
pub fn load_page(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent from form navigating to sommewhere else
	event.prevent_default();

	// getting page input
	let page_value = event.target() // getting form
	                      .ok_or("event should have form in target")?
	                      .dyn_into::<web_sys::HtmlFormElement>()?
	                      .get_with_name("page") // get input field
	                      .dyn_into::<web_sys::HtmlInputElement>()?
	                      .value_as_number() // get value
	                      as usize; // convert to int

	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// get table
	let word_table = document
		.get_element_by_id("word_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?;

	// clear table
	for _ in 0..word_table.rows().length() {
		word_table.delete_row(-1)?;
	}

	// prepare word
	let mut word = Word::default();

	// fix word up for desired page

	// fix first two letters according to page
	for page in 1..page_value {
		// increment fist letter every time we switch the first letter
		// which is full length -1
		if page % (Word::CONSONANTS.len() - 1) == 0 {
			// increment first letter
			word.increment_letter(0);
		}
		// otherwise increment second letter
		else {
			word.increment_letter(1);
		}
	}

	// now that the first word is fixed, generate all words for the page
	for _ in 0..Word::CONSONANTS.len() - 2 {
		// generate row
		let row = word_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;

		for _ in 0..Word::CONSONANTS.len() - 3 {
			// print word
			let link = document.create_element("a")?.dyn_into::<web_sys::HtmlElement>()?;
			link.set_inner_text(&word.to_string());
			link.set_attribute(
				"href",
				&format!(
					"javascript:wasm_bindgen.display_forms({}, {}, {}, {})",
					word.get_id()[0],
					word.get_id()[1],
					word.get_id()[2],
					word.get_id()[3]
				)
			)?;
			row.insert_cell()?.append_child(&link)?;

			// get next word
			word.increment_letter(3);
		}

		// get next word
		word.increment_letter(2);
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn check_word(event: &web_sys::Event) -> Result<(), JsValue> {
	// getting input field
	let input = event
		.target()
		.ok_or("event should have input in target")?
		.dyn_into::<web_sys::HtmlInputElement>()?;
	// get value
	let word = input.value();

	// reset checks
	input.set_custom_validity("");

	// check if html checks are passed
	if input.check_validity() {
		// check if word was successfully generated
		match Word::from_string(&word) {
			Ok(word) => {
				let word = word.get_id();
				return display_forms(word[0], word[1], word[2], word[3]);
			},
			// otherwise return error
			Err(err) => {
				input.set_custom_validity(err);
				return Ok(());
			}
		}
	}
	// if it didn't pass html check, go away!
	else {
		return Ok(());
	}
}

#[wasm_bindgen]
pub fn load_word(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent from form navigating to sommewhere else
	event.prevent_default();

	// getting page input
	let word = event
		.target() // getting form
		.ok_or("event should have form in target")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("word") // get input field
		.dyn_into::<web_sys::HtmlInputElement>()?
		.value(); // get value

	if let Ok(word) = Word::from_string(&word) {
		let word = word.get_id();
		return display_forms(word[0], word[1], word[2], word[3]);
	}
	else {
		return Err("asd".into());
	}
}

#[wasm_bindgen]
pub fn display_forms(id_0: usize, id_1: usize, id_2: usize, id_3: usize) -> Result<(), JsValue> {
	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// get table
	let word_table = document
		.get_element_by_id("word_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?;

	// clear table
	for _ in 0..word_table.rows().length() {
		word_table.delete_row(-1)?;
	}

	// prepare forms
	let forms = Word::from_id([id_0, id_1, id_2, id_3]).generate_forms();

	for form in &forms {
		let row = word_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;

		for form in form {
			row.insert_cell()?.set_inner_text(&form);
		}
	}

	return Ok(());
}
