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

//! This is a simple wasm powered page that generates all possible words of the UYWI language.

mod util;

use seed::{
	prelude::{AsAtValue, At, El, IndexMap, Node, Orders, UpdateEl, View},
	App,
};
use std::str::FromStr;
use util::*;
use uywi::{Concept, Length, Page};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{FormData, HtmlInputElement};

/// Data we save for the app.
#[derive(Default)]
struct Model {
	/// Current concept length.
	length: Length,
	/// Which type of data we are currently showing.
	view: ModelView,
}

/// Represents which type of data we are currently showing.
enum ModelView {
	/// Show a page.
	Page {
		/// The `Page`.
		page: Page,
	},
	/// Show a page with a highlighted concept.
	Concept {
		/// The `Page`.
		page: Page,
		/// The `Concept`.
		concept: Concept,
	},
	/// Show forms.
	Forms {
		/// The `Concept`.
		concept: Concept,
	},
}

impl Default for ModelView {
	fn default() -> Self {
		return Self::Page {
			page: Page::from_index(0, Length::L4).expect("failed to create page"),
		};
	}
}

impl ModelView {
	/// Convert all data to strings. Needed to fill forms.
	fn data_to_string(&self) -> ModelDataString {
		let (page, concept, index) = match self {
			Self::Page { page } => (page.to_string(), "".into(), "".into()),
			Self::Concept { page, concept } => (page.to_string(), concept.to_string(), concept.index_as_string()),
			Self::Forms { concept } => ("".into(), concept.to_string(), concept.index_as_string()),
		};

		return ModelDataString { page, concept, index };
	}
}

/// Just a convenience structure to pass along data from `Model` as strings.
struct ModelDataString {
	/// The `Page` but as a `String`.
	page: String,
	/// The `Concept` but as a `String`.
	concept: String,
	/// The `Concept` index but as a `String`.
	index: String,
}

/// Drawing app.
fn view(model: &Model) -> impl View<Event> {
	use seed::{attrs, button, class, div, form, input, option, select};

	// get all data as `String`s
	let ModelDataString { page, concept, index } = model.view.data_to_string();

	return vec![
		div![
			class!["table"],
			// concept length
			form![
				class!["tr"],
				util::submit_ev(Event::Length),
				div![class!["td"], "Length:"],
				div![
					class!["td"],
					select![
						attrs![At::Name => "input"],
						option![
							attrs![At::Value => Length::L2, At::Selected => (Length::L2 == model.length).as_at_value()],
							Length::L2.to_string()
						],
						option![
							attrs![At::Value => Length::L3, At::Selected => (Length::L3 == model.length).as_at_value()],
							Length::L3.to_string()
						],
						option![
							attrs![At::Value => Length::L4, At::Selected => (Length::L4 == model.length).as_at_value()],
							Length::L4.to_string()
						],
					],
				],
				div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
			],
			// page input
			form![
				class!["tr"],
				util::submit_ev(Event::Page),
				div![class!["td"], "Page:"],
				div![
					class!["td"],
					input![attrs![
						At::Type => "number",
						At::Name => "input",
						At::Required => true.as_at_value(),
						At::Min => 1,
						At::Max => model.length.num_of_pages(),
						At::Value => page
					]],
				],
				div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
			],
			// concept string input
			form![
				class!["tr"],
				util::submit_ev(Event::Concept),
				div![class!["td"], "Concept:"],
				div![
					class!["td"],
					input![
						attrs![
							At::Type => "text",
							At::Name => "input",
							At::Required => true.as_at_value(),
							At::Custom("minlength".into()) => model.length.as_int(),
							At::MaxLength => model.length.as_int(),
							At::Value => concept
						],
						util::input_ev(Event::ConceptInput)
					],
				],
				div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
			],
			// concept index input
			form![
				class!["tr"],
				util::submit_ev(Event::Index),
				div![class!["td"], "Index:"],
				div![
					class!["td"],
					input![attrs![
						At::Type => "number",
						At::Name => "input",
						At::Required => true.as_at_value(),
						At::Min => 1,
						At::Max => model.length.num_of_concepts(),
						At::Value => index
					]],
				],
				div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
			],
		],
		// displayed table
		create_table(model),
	];
}

/// Create the table.
fn create_table(model: &Model) -> Node<Event> {
	use seed::{a, attrs, table, td, tr};

	let mut table = table![];

	match model.view {
		// `ModelView::Page` and `ModelView::Concept` both show concepts
		ModelView::Page { page } | ModelView::Concept { page, .. } => {
			// reserve appropriate space for the table
			table.reserve_children(model.length.rows_per_page());

			// only `ModelView::Concept` has a concept to highlight
			let highlighted_concept = if let ModelView::Concept { concept, .. } = model.view {
				Some(concept)
			} else {
				None
			};

			for row in page {
				let mut html_row = tr![];

				// reserve appropriate space for the row
				html_row.reserve_children(model.length.concepts_per_row());

				for concept in row {
					let mut cell = td![];

					// check for the highlighted concept
					if let Some(highlighted_concept) = highlighted_concept {
						if highlighted_concept == concept {
							cell.add_class("highlighted");
						}
					}

					// add a link that leads to `Event::OpenConcept`
					cell.add_child(a![attrs! [At::Href => "#"], click_ev(concept, Event::OpenConcept), concept.to_string()]);

					html_row.add_child(cell);
				}

				table.add_child(html_row);
			}
		},
		// show forms
		ModelView::Forms { concept } => {
			// reserve appropriate space for the table
			table.reserve_children(model.length.stems_per_concept());

			for stem in concept {
				let mut row = tr![];

				// reserve appropriate space for the row
				row.reserve_children(model.length.forms_per_stem());

				for form in stem {
					row.add_child(td![form.to_string()]);
				}

				table.add_child(row);
			}
		},
	}

	return table;
}

/// Events that are sent by the app.
#[derive(Debug, Clone)]
enum Event {
	/// Handle concept length.
	Length(FormData),
	/// Handle page form.
	Page(FormData),
	/// Handle concept form.
	Concept(FormData),
	/// Check concept input for validity.
	ConceptInput(HtmlInputElement, String),
	/// Handle index form.
	Index(FormData),
	/// Handle clicking on a concept.
	OpenConcept(Concept),
}

/// Handling events.
fn update(event: Event, model: &mut Model, orders: &mut impl Orders<Event>) {
	match event {
		Event::Length(data) => {
			// set length
			let data = data.pget("input").parse().expect("length input isn't an integer");
			model.length = Length::new(data).expect("length input is invalid");

			// reset data
			model.view = ModelView::Page {
				page: Page::from_index(0, model.length).expect("failed to create page"),
			};
		},
		// handle page form
		Event::Page(data) => {
			let page = Page::from_str(&data.pget("input"), model.length).expect("failed to create page");

			model.view = ModelView::Page { page };
		},
		// handle concept string and index input
		event @ Event::Concept(..) | event @ Event::Index(..) => {
			// extract `Concept` from form
			#[allow(clippy::wildcard_enum_match_arm)]
			let concept = match event {
				Event::Concept(data) => Concept::from_str(&data.pget("input")),
				Event::Index(data) => Concept::from_index_str(&data.pget("input"), model.length),
				_ => unreachable!("filtered event still reached"),
			};

			let concept = concept.expect("failed to create `Concept`");

			model.view = ModelView::Concept {
				page: concept.page(),
				concept,
			};
		},
		// handle concept string input
		Event::ConceptInput(input, value) => {
			// reset checks, because `check_validity` will trigger the last error
			input.set_custom_validity("");

			// make sure html checks are passed
			if input.check_validity() {
				// we want to display custom error messages if concept wasn't generated
				if let Err(error) = Concept::from_str(&value) {
					input.set_custom_validity(&error.to_string());
					// if there is an error skip rendering
					orders.skip();
				}
			}
		},
		// handle clicking on a link to forms
		Event::OpenConcept(concept) => {
			model.view = ModelView::Forms { concept };
		},
	}
}

#[wasm_bindgen(start)]
pub fn main() {
	#[cfg(debug_assertions)]
	{
		use log::Level;
		use std::f32;
		use wasm_logger::{init, Config};

		// increase stacktrace limit
		util::set_stacktracelimit(f32::INFINITY);

		// initialize logging
		init(Config::new(Level::Trace).message_on_new_line());
	}

	App::build(|_, _| return Model::default(), update, view)
		.mount(seed::body())
		.finish()
		.run();
}
