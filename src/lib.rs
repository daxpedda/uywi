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
pub fn onsubmit(event: &web_sys::Event) -> Result<(), JsValue> {
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
			row.insert_cell()?.set_inner_text(word.to_string().as_str());

			// get next word
			word.increment_letter(3);
		}

		// get next word
		word.increment_letter(2);
	}

	return Ok(());
}
