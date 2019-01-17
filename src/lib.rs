//#![no_std]
#![feature(type_ascription, try_blocks, alloc, slice_concat_ext)]
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
use wasm_bindgen::{prelude::*, JsCast};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

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
pub fn display_page(event: &web_sys::Event) -> Result<(), JsValue> {
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

	return load_page(page_value, &None);
}

fn load_page(page_value: usize, highlighted_word: &Option<Word>) -> Result<(), JsValue> {
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

	// calculate amount of words we skipped due to pages
	let words_skipped = (page_value - 1) * ((Word::CONSONANTS.len() - 2) * (Word::CONSONANTS.len() - 3));
	// prepare word
	let mut word = Word::from_word_index(words_skipped);

	// now that the first word is fixed, generate all words for the page
	for _ in 0..Word::CONSONANTS.len() - 2 {
		// generate row
		let row = word_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;

		for _ in 0..Word::CONSONANTS.len() - 3 {
			// print word
			let link = document.create_element("a")?.dyn_into::<web_sys::HtmlElement>()?;
			link.set_inner_text(&word.to_string());
			link.set_attribute("href", "#")?;
			let index = word.get_word_index();
			let onclick_closure = Closure::wrap(
				Box::new(move |event: web_sys::Event| return display_forms(&event, index)) as Box<Fn(web_sys::Event) -> Result<(), JsValue>>
			);
			link.set_onclick(Some(onclick_closure.as_ref().unchecked_ref()));
			onclick_closure.forget();
			let cell = row.insert_cell()?;
			cell.append_child(&link)?;

			if let Some(ref highlighted_word) = highlighted_word {
				if highlighted_word == &word {
					cell.style().set_property("background-color", "yellow")?;
				}
			}

			// get next word
			word.increment_word();
		}
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn display_word_by_word(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent form from navigating to sommewhere else
	event.prevent_default();

	// getting input field
	let input = event
		.target() // getting form
		.ok_or("event should have form in target")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("word") // get input field
		.dyn_into::<web_sys::HtmlInputElement>()?;
	// get value
	let word = input.value();

	match Word::from_string(&word) {
		// if everything checks out, display page!
		Ok(word) => {
			let index = word.get_word_index();
			let pages = index / ((Word::CONSONANTS.len() - 2) * (Word::CONSONANTS.len() - 3)) + 1;

			web_sys::window()
				.ok_or("window should exist")?
				.document()
				.ok_or("should have a document on window")?
				.forms()
				.get_with_index(0) // get only form on page
				.ok_or("should have a form")?
				.dyn_into::<web_sys::HtmlFormElement>()?
				.get_with_name("page") // get page input
				.dyn_into::<web_sys::HtmlInputElement>()?
				.set_value_as_number(pages as f64); // clicking on it triggers onsubmit(event)

			web_sys::window()
				.ok_or("window should exist")?
				.document()
				.ok_or("should have a document on window")?
				.forms()
				.get_with_index(2) // get only form on page
				.ok_or("should have a form")?
				.dyn_into::<web_sys::HtmlFormElement>()?
				.get_with_name("index") // get page input
				.dyn_into::<web_sys::HtmlInputElement>()?
				.set_value_as_number((index + 1) as f64); // clicking on it triggers onsubmit(event)

			return load_page(pages, &Some(word));
		},
		// we want to display custom error messages if word wasn't generated
		Err(err) => input.set_custom_validity(err)
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn display_word_by_index(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent form from navigating to sommewhere else
	event.prevent_default();

	// getting input field
	let index = event
		.target() // getting form
		.ok_or("event should have form in target")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("index") // get input field
		.dyn_into::<web_sys::HtmlInputElement>()?
		.value_as_number() as usize
		- 1;

	let word = Word::from_word_index(index);
	let pages = index / ((Word::CONSONANTS.len() - 2) * (Word::CONSONANTS.len() - 3)) + 1;

	web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?
		.forms()
		.get_with_index(0) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("page") // get page input
		.dyn_into::<web_sys::HtmlInputElement>()?
		.set_value_as_number(pages as f64); // clicking on it triggers onsubmit(event)

	return load_page(pages, &Some(word));
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
		// we want to display custom error messages if word wasn't generated
		if let Err(err) = Word::from_string(&word) {
			input.set_custom_validity(err);
		}
	}

	return Ok(());
}

pub fn display_forms(event: &web_sys::Event, index: usize) -> Result<(), JsValue> {
	// prevent link from navigating to sommewhere else
	event.prevent_default();

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
	let forms = Word::from_word_index(index).generate_forms();

	web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?
		.forms()
		.get_with_index(2) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("index") // get page input
		.dyn_into::<web_sys::HtmlInputElement>()?
		.set_value_as_number((index + 1) as f64); // clicking on it triggers onsubmit(event)

	for form in &forms {
		let row = word_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;

		for form in form {
			row.insert_cell()?.set_inner_text(&form);
		}
	}

	return Ok(());
}
