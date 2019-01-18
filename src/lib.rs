//#![no_std]
#![feature(type_ascription, try_blocks, alloc, slice_concat_ext, const_slice_len, maybe_uninit)]
#![warn(
	clippy::cargo, // rls being weird
	clippy::pedantic,
	clippy::nursery,
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

extern crate alloc;

mod concept;

use alloc::boxed::Box;
use concept::Concept;
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

	web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?
		.forms()
		.get_with_index(1) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("concept") // get page input
		.dyn_into::<web_sys::HtmlInputElement>()?
		.set_value(""); // clicking on it triggers onsubmit(event)

	return load_page(page_value, &None);
}

fn load_page(page_value: usize, highlighted_concept: &Option<Concept>) -> Result<(), JsValue> {
	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// hide intonation table
	document
		.get_element_by_id("intonation_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?
		.style()
		.set_property("display", "none")?;

	// get table
	let concept_table = document
		.get_element_by_id("concept_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?;

	// clear table
	for _ in 0..concept_table.rows().length() {
		concept_table.delete_row(-1)?;
	}

	// calculate amount of concepts we skipped due to pages
	let concepts_skipped = (page_value - 1) * ((Concept::RADICALS.len() - 2) * (Concept::RADICALS.len() - 3));
	// prepare concept
	let mut concept = Concept::from_concept_index(concepts_skipped);

	// now that the first concept is fixed, generate all concepts for the page
	for _ in 0..Concept::RADICALS.len() - 2 {
		// generate row
		let row = concept_table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;

		for _ in 0..Concept::RADICALS.len() - 3 {
			// print concept
			let link = document.create_element("a")?.dyn_into::<web_sys::HtmlElement>()?;
			link.set_inner_text(&concept.to_string());
			link.set_attribute("href", "#")?;
			let index = concept.get_concept_index();
			let onclick_closure =
				Closure::wrap(Box::new(move |event: web_sys::Event| return display_stems(&event, index, None))
					as Box<Fn(web_sys::Event) -> Result<(), JsValue>>);
			link.set_onclick(Some(onclick_closure.as_ref().unchecked_ref()));
			onclick_closure.forget();
			let cell = row.insert_cell()?;
			cell.append_child(&link)?;

			if let Some(ref highlighted_concept) = highlighted_concept {
				if highlighted_concept == &concept {
					cell.style().set_property("background-color", "yellow")?;
				}
			}

			// get next concept
			concept.increment_concept();
		}
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn display_concept_by_concept(event: &web_sys::Event) -> Result<(), JsValue> {
	// prevent form from navigating to sommewhere else
	event.prevent_default();

	// getting input field
	let input = event
		.target() // getting form
		.ok_or("event should have form in target")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("concept") // get input field
		.dyn_into::<web_sys::HtmlInputElement>()?;
	// get value
	let concept_string = input.value();

	match Concept::from_string(&concept_string) {
		// if everything checks out, display page!
		Ok(concept) => {
			let index = concept.get_concept_index();
			let pages = index / ((Concept::RADICALS.len() - 2) * (Concept::RADICALS.len() - 3)) + 1;

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

			return load_page(pages, &Some(concept));
		},
		// we want to display custom error messages if concept wasn't generated
		Err(err) => input.set_custom_validity(err)
	}

	return Ok(());
}

#[wasm_bindgen]
pub fn display_concept_by_index(event: &web_sys::Event) -> Result<(), JsValue> {
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

	let concept = Concept::from_concept_index(index);
	let pages = index / ((Concept::RADICALS.len() - 2) * (Concept::RADICALS.len() - 3)) + 1;

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
		.get_with_index(1) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("concept") // get page input
		.dyn_into::<web_sys::HtmlInputElement>()?
		.set_value(&concept.to_string()); // clicking on it triggers onsubmit(event)

	return load_page(pages, &Some(concept));
}

#[wasm_bindgen]
pub fn check_concept(event: &web_sys::Event) -> Result<(), JsValue> {
	// getting input field
	let input = event
		.target()
		.ok_or("event should have input in target")?
		.dyn_into::<web_sys::HtmlInputElement>()?;
	// get value
	let concept = input.value();

	// reset checks
	input.set_custom_validity("");

	// check if html checks are passed
	if input.check_validity() {
		// we want to display custom error messages if concept wasn't generated
		if let Err(err) = Concept::from_string(&concept) {
			input.set_custom_validity(err);
		}
	}

	return Ok(());
}

fn display_stems(event: &web_sys::Event, index: usize, highlighted_stem: Option<(usize, usize)>) -> Result<(), JsValue> {
	// prevent link from navigating to sommewhere else
	event.prevent_default();

	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// hide intonation table
	document
		.get_element_by_id("intonation_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?
		.style()
		.set_property("display", "none")?;

	// get table
	let concept_table = document
		.get_element_by_id("concept_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?;

	// clear table
	for _ in 0..concept_table.rows().length() {
		concept_table.delete_row(-1)?;
	}

	// prepare stems
	let concept = Concept::from_concept_index(index);
	let stems = concept.generate_stems();

	web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?
		.forms()
		.get_with_index(1) // get only form on page
		.ok_or("should have a form")?
		.dyn_into::<web_sys::HtmlFormElement>()?
		.get_with_name("concept") // get page input
		.dyn_into::<web_sys::HtmlInputElement>()?
		.set_value(&concept.to_string()); // clicking on it triggers onsubmit(event)

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

	let mut rows: [Result<Result<web_sys::HtmlTableRowElement, web_sys::HtmlElement>, JsValue>; Concept::STEMS.len()] =
		array_init::array_init(|_| {
			return concept_table
				.insert_row()
				.map(|row| return row.dyn_into::<web_sys::HtmlTableRowElement>());
		});

	for (index_form, form) in stems.iter().enumerate() {
		for (index_stem, (stem, row)) in form.iter().zip(rows.iter_mut()).enumerate() {
			let row = row.as_ref()?.as_ref()?;
			let link = document.create_element("a")?.dyn_into::<web_sys::HtmlElement>()?;
			link.set_inner_text(stem);
			link.set_attribute("href", "#")?;
			let onclick_closure =
				Closure::wrap(
					Box::new(move |event: web_sys::Event| return display_intonations(&event, index, index_form, index_stem))
						as Box<Fn(web_sys::Event) -> Result<(), JsValue>>
				);
			link.set_onclick(Some(onclick_closure.as_ref().unchecked_ref()));
			onclick_closure.forget();
			let cell = row.insert_cell()?;
			cell.append_child(&link)?;

			if let Some((highlighted_form, highlighted_stem)) = highlighted_stem {
				if highlighted_form == index_form && highlighted_stem == index_stem {
					cell.style().set_property("background-color", "yellow")?;
				}
			}
		}
	}

	return Ok(());
}

fn display_intonations(event: &web_sys::Event, index: usize, index_form: usize, index_stem: usize) -> Result<(), JsValue> {
	// prevent link from navigating to sommewhere else
	event.prevent_default();

	display_stems(event, index, Some((index_form, index_stem)))?;

	// get document
	let document = web_sys::window()
		.ok_or("window should exist")?
		.document()
		.ok_or("should have a document on window")?;

	// get table
	let intonation_table = document
		.get_element_by_id("intonation_table")
		.ok_or("table should exist")?
		.dyn_into::<web_sys::HtmlTableElement>()?;

	intonation_table.style().set_property("display", "inline-table")?;

	// clear table
	for _ in 0..intonation_table.rows().length() {
		intonation_table.delete_row(-1)?;
	}

	// prepare forms
	let intonations = Concept::from_concept_index(index).generate_intonations(index_form, index_stem);

	for intonation in &intonations {
		intonation_table
			.insert_row()?
			.dyn_into::<web_sys::HtmlTableRowElement>()?
			.insert_cell()?
			.set_inner_text(intonation);
	}

	return Ok(());
}
