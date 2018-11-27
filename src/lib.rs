#![feature(type_ascription)]
#![feature(try_blocks)]
#![warn(
	//clippy::cargo, // rls being weird
	clippy::pedantic,
	clippy::restriction
)]
#![allow(
	clippy::needless_return,
	clippy::non_ascii_literal,
	clippy::decimal_literal_representation,
	clippy::float_arithmetic,
	clippy::integer_arithmetic,	// clippy being stupid
	clippy::items_after_statements,	// clippy being weird
	clippy::missing_docs_in_private_items, // rls being weird
	clippy::missing_inline_in_public_items,
	clippy::print_stdout, // rls being weird
	clippy::shadow_reuse,
	clippy::shadow_same
)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn onload() -> Result<(), JsValue> {
	// get some basics
	let document = web_sys::window().expect("window should exist")
	                                .document()
	                                .expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	// form to put page stuff inside
	let page_form = document.create_element("form")?.dyn_into::<web_sys::HtmlFormElement>()?;
	let onsubmit_closure = Closure::wrap(Box::new(|event: web_sys::Event| onsubmit(&event)) as Box<Fn(web_sys::Event) -> Result<(), JsValue>>);
	page_form.set_onsubmit(Some(onsubmit_closure.as_ref().unchecked_ref()));
	onsubmit_closure.forget();
	body.append_child(&page_form)?;
	// page text
	let page_text = document.create_element("span")?.dyn_into::<web_sys::HtmlSpanElement>()?;
	page_text.set_inner_text("Page: ");
	page_form.append_child(&page_text)?;
	// page input box
	let page_input = document.create_element("input")?.dyn_into::<web_sys::HtmlInputElement>()?;
	page_input.set_type("number");
	page_input.set_name("page");
	page_input.set_value_as_number(1.);
	page_input.set_required(true);
	page_input.set_min("1");
	page_input.set_max("992");
	page_input.style().set_property("width", "3rem")?;
	page_form.append_child(&page_input)?;
	// page spacce between input and button
	let page_spacing = document.create_element("span")?.dyn_into::<web_sys::HtmlSpanElement>()?;
	page_spacing.set_inner_text(" ");
	page_form.append_child(&page_spacing)?;
	// page load
	let page_load = document.create_element("button")?.dyn_into::<web_sys::HtmlButtonElement>()?;
	page_load.set_type("submit");
	page_load.set_inner_text("Load");
	page_form.append_child(&page_load)?;

	// table with words
	let word_table = document.create_element("table")?.dyn_into::<web_sys::HtmlTableElement>()?;
	word_table.set_id("word_table");
	word_table.set_border("1");
	body.append_child(&word_table)?;

	// load page at start
	page_load.click();

	return Ok(());
}

fn onsubmit(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent from form navigating to sommewhere else
	event.prevent_default();

	// getting page input
	let page_value = event.target() // getting form
	                      .expect("event should have form in target")
	                      .dyn_into::<web_sys::HtmlFormElement>()?
	                      .get_with_name("page") // get input field
	                      .dyn_into::<web_sys::HtmlInputElement>()?
	                      .value() // get value
	                      .parse() // convert to int
	                      .expect("should only be an integer"): usize;

	// get document
	let document = web_sys::window().expect("window should exist")
	                                .document()
	                                .expect("should have a document on window");

	// get table
	let word_table = document.get_element_by_id("word_table")
	                         .expect("table should exist")
	                         .dyn_into::<web_sys::HtmlTableElement>()?;

	// clear table
	for _ in 0..word_table.rows().length() {
		word_table.delete_row(-1)?;
	}

	let consonants_table = [
	                        '?', 'y', 'w', 'h', '2', '7', 'q', '5', 'k', '8', 'g', 'j', '3', '9', 'S', 's', 'Z', 'z', 'D', 'd', 'T', 't', '0', '6',
	                        'f', 'b', 'p', 'm', 'n', 'o', 'r', 'l',
	];

	let mut word = [3, 0, 2, 1];

	let count_up = |word: &mut [usize; 4], index: usize| loop {
		if (index > 0 && word[0] == word[index])
		   || (index > 1 && word[1] == word[index])
		   || (index > 2 && word[2] == word[index])
		   || (index > 3 && word[3] == word[index])
		{
			word[index] += 1;
		}
		else if word[index] == 32 {
			word[index] = 0;
		}
		else {
			break;
		}
	};

	for iteration in 1..page_value {
		word[1] += 1;

		if iteration % 31 == 0 {
			word[3] = 1;
			word[2] = 2;
			word[1] = 0;
			word[0] += 1;
			count_up(&mut word, 0);
		}

		count_up(&mut word, 1);
	}

	for _ in 0..30 {
		let row = word_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;
		count_up(&mut word, 2);

		for _ in 0..29 {
			count_up(&mut word, 3);

			row.insert_cell()?.set_inner_text(format!(
				"{}{}{}{}",
				consonants_table.get(word[1]).ok_or(0)?,
				consonants_table.get(word[3]).ok_or(0)?,
				consonants_table.get(word[2]).ok_or(0)?,
				consonants_table.get(word[0]).ok_or(0)?
			).as_str());

			word[3] += 1;
		}

		word[3] = 1;
		word[2] += 1;
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
	// get document
	let document = web_sys::window().expect("window should exist")
	                                .document()
	                                .expect("should have a document on window");

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
