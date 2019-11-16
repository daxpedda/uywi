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
	prelude::{AsAtValue, At, El, IndexMap, Init, Node, Orders, UpdateEl, View},
	App,
};
use util::*;
use uywi::{Accent, Concept, Length, Page};
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

/// Data we save for the app.
#[derive(Default)]
struct Model {
	/// Current accent.
	accent: Accent,
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
		/// The `Page`.
		page: Page,
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
			Self::Concept { page, concept } | Self::Forms { page, concept } => {
				(page.to_string(), concept.to_string(Accent::UywiChiffre), concept.index_as_string())
			},
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
			// accent
			build_accent(model),
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
							At::Size => 1,
							At::Required => true.as_at_value(),
							At::MinLength => 2,
							At::MaxLength => 4,
							At::AutoComplete => "off",
							At::SpellCheck => "false",
							At::Custom("autocorrect".into()) => "off",
							At::Custom("autocapitalize".into()) => "off",
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
		build_table(model),
	];
}

/// Build accent form.
fn build_accent(model: &Model) -> Node<Event> {
	use seed::{attrs, button, class, div, form, option, select};

	return form![
		class!["tr"],
		util::submit_ev(Event::Accent),
		div![class!["td"], "Accent:"],
		div![
			class!["td"],
			select![
				attrs![At::Name => "input"],
				option![
					attrs![At::Value => 0, At::Selected => (Accent::UywiChiffre == model.accent).as_at_value()],
					Accent::UywiChiffre.to_string()
				],
				option![
					attrs![At::Value => 1, At::Selected => (Accent::IpaPeter == model.accent).as_at_value()],
					Accent::IpaPeter.to_string()
				],
			],
		],
		div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
	];
}

/// Create the table.
fn build_table(model: &Model) -> Node<Event> {
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
					cell.add_child(a![
						attrs! [At::Href => "#"],
						click_ev(concept, Event::OpenConcept),
						concept.to_string(model.accent)
					]);

					html_row.add_child(cell);
				}

				table.add_child(html_row);
			}
		},
		// show forms
		ModelView::Forms { concept, .. } => {
			// reserve appropriate space for the table
			table.reserve_children(model.length.stems_per_concept());

			for stem in concept {
				let mut row = tr![];

				// reserve appropriate space for the row
				row.reserve_children(model.length.words_per_stem());

				for form in stem {
					row.add_child(td![form.to_string(model.accent)]);
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
	/// Handle accent.
	Accent(HtmlFormElement, FormData),
	/// Handle concept length.
	Length(HtmlFormElement, FormData),
	/// Handle page form.
	Page(HtmlFormElement, FormData),
	/// Handle concept form.
	Concept(HtmlFormElement, FormData),
	/// Check concept input for validity.
	ConceptInput(HtmlInputElement, String),
	/// Handle index form.
	Index(HtmlFormElement, FormData),
	/// Handle clicking on a concept.
	OpenConcept(Concept),
}

/// Handling events.
fn update(event: Event, model: &mut Model, orders: &mut impl Orders<Event>) {
	match event {
		Event::Accent(_, data) => {
			// set accent
			let data = data.pget("input").parse().expect("accent input isn't an integer");
			model.accent = match data {
				0 => Accent::UywiChiffre,
				1 => Accent::IpaPeter,
				_ => unreachable!("invalid accent"),
			}
		},
		Event::Length(_, data) => {
			// set length
			let data = data.pget("input").parse().expect("length input isn't an integer");
			model.length = Length::new(data).expect("length input is invalid");

			// reset data
			model.view = ModelView::Page {
				page: Page::from_index(0, model.length).expect("failed to create page"),
			};
		},
		// handle page form
		Event::Page(_, data) => {
			let page = Page::from_str(&data.pget("input"), model.length).expect("failed to create page");

			model.view = ModelView::Page { page };
		},
		// handle concept string
		Event::Concept(form, data) => {
			// get input
			let input = form.pget::<HtmlInputElement>("input");

			// reset checks, because `check_validity` will trigger the last error
			input.set_custom_validity("");

			// make sure html checks are passed
			if input.check_validity() {
				match Concept::from_str(&data.pget("input"), Accent::UywiChiffre) {
					Ok(concept) => {
						model.length = concept.length();
						model.view = ModelView::Concept {
							page: concept.page(),
							concept,
						};
					},
					Err(error) => {
						input.set_custom_validity(&error.to_string());
						// if there is an error skip rendering
						orders.skip();
					},
				}
			// skip if html chehcks didnt pass
			} else {
				orders.skip();
			}
		},
		// handle concept index
		Event::Index(_, data) => {
			let concept = Concept::from_index_str(&data.pget("input"), model.length).expect("failed to create `Concept`");

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
				if let Err(error) = Concept::from_str(&value, Accent::UywiChiffre) {
					input.set_custom_validity(&error.to_string());
					// if there is an error skip rendering
					orders.skip();
				}
			// skip if html chehcks didnt pass
			} else {
				orders.skip();
			}
		},
		// handle clicking on a link to forms
		Event::OpenConcept(concept) => {
			model.view = ModelView::Forms {
				page: concept.page(),
				concept,
			};
		},
	}
}

fn main() {
	#[cfg(debug_assertions)]
	{
		use log::Level;
		use std::f32;
		use wasm_logger::{init, Config};

		// increase stacktrace limit
		util::set_stacktracelimit(f32::INFINITY);

		// initialize logging
		init(Config::new(Level::max()).message_on_new_line());
	}

	App::build(|_, _| return Init::new(Model::default()), update, view)
		.mount(seed::body())
		.build_and_start();
}
