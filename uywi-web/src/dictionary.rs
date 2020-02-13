//! Generates all possible words of the UYWI language.

use crate::{
	home::State as Home,
	util::{self, *},
	Event as SuperEvent, State as SuperState,
};
use seed::prelude::{AsAtValue, At, El, Ev, IndexMap, Node, Orders, UpdateEl};
use uywi::{Concept, Length, Page, Script};
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};

/// State of the dictionary.
#[derive(Default)]
pub(crate) struct State {
	/// Current script.
	script: Script,
	/// Current concept length.
	length: Length,
	/// Which type of data we are currently showing.
	view: StateView,
}

/// Represents which type of data we are currently showing.
enum StateView {
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

impl Default for StateView {
	fn default() -> Self {
		return Self::Page {
			page: Page::from_index(0, Length::L4).expect("failed to create page"),
		};
	}
}

impl StateView {
	/// Convert all data to strings. Needed to fill forms.
	fn data_to_string(&self) -> StateDataString {
		let (page, concept, index) = match self {
			Self::Page { page } => (page.to_string(), "".into(), "".into()),
			Self::Concept { page, concept } | Self::Forms { page, concept } => {
				(page.to_string(), concept.to_string(Script::UywiChiffre), concept.index_as_string())
			},
		};

		return StateDataString { page, concept, index };
	}
}

/// Just a convenience structure to pass along data from [`State`] as strings.
struct StateDataString {
	/// The `Page` but as a `String`.
	page: String,
	/// The `Concept` but as a `String`.
	concept: String,
	/// The `Concept` index but as a `String`.
	index: String,
}

/// Events that are sent by the app.
#[derive(Debug, Clone)]
pub(crate) enum Event {
	/// Go back to the homescreen.
	Home,
	/// Handle script.
	Script(HtmlFormElement, FormData),
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

impl State {
	/// Drawing app.
	pub(crate) fn view(&self) -> Vec<Node<SuperEvent>> {
		use seed::{attrs, button, class, div, form, input, option, prelude::ev, select};

		// get all data as `String`s
		let StateDataString { page, concept, index } = self.view.data_to_string();

		return vec![
			div![
				class!["table"],
				// go back to homescreen
				div![
					class!["tr"],
					div![
						class!["td"],
						attrs![At::ColSpan => 3],
						button![attrs![At::Type => "button"], ev(Ev::Click, |_| return Event::Home.into()), "Back to home"],
					]
				],
				// script
				build_script(self),
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
								attrs![At::Value => Length::L2, At::Selected => (Length::L2 == self.length).as_at_value()],
								Length::L2.to_string()
							],
							option![
								attrs![At::Value => Length::L3, At::Selected => (Length::L3 == self.length).as_at_value()],
								Length::L3.to_string()
							],
							option![
								attrs![At::Value => Length::L4, At::Selected => (Length::L4 == self.length).as_at_value()],
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
							At::Max => self.length.num_of_pages(),
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
							At::Max => self.length.num_of_concepts(),
							At::Value => index
						]],
					],
					div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
				],
			],
			// displayed table
			build_table(self),
		];
	}

	/// Handling events.
	pub(crate) fn update(&mut self, event: Event, orders: &mut impl Orders<SuperEvent>) -> Option<SuperState> {
		match event {
			Event::Home => return Some(SuperState::Home(Home::default())),
			Event::Script(_, data) => {
				// set script
				let data = data.pget("input").parse().expect("script input isn't an integer");
				self.script = match data {
					0 => Script::UywiChiffre,
					1 => Script::IpaPeter,
					_ => unreachable!("invalid script"),
				}
			},
			Event::Length(_, data) => {
				// set length
				let data = data.pget("input").parse().expect("length input isn't an integer");
				self.length = Length::new(data).expect("length input is invalid");

				// reset data
				self.view = StateView::Page {
					page: Page::from_index(0, self.length).expect("failed to create page"),
				};
			},
			// handle page form
			Event::Page(_, data) => {
				let page = Page::from_str(&data.pget("input"), self.length).expect("failed to create page");

				self.view = StateView::Page { page };
			},
			// handle concept string
			Event::Concept(form, data) => {
				// get input
				let input = form.pget::<HtmlInputElement>("input");

				// reset checks, because `check_validity` will trigger the last error
				input.set_custom_validity("");

				// make sure html checks are passed
				if input.check_validity() {
					match Script::UywiChiffre.from_concept(&data.pget("input")) {
						Ok(concept) => {
							self.length = concept.length();
							self.view = StateView::Concept {
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
				let concept = Concept::from_index_str(&data.pget("input"), self.length).expect("failed to create `Concept`");

				self.view = StateView::Concept {
					page: concept.page(),
					concept,
				};
			},
			// handle concept string input
			Event::ConceptInput(input, value) => {
				// we don't want to rerender anything on input
				orders.skip();

				// reset checks, because `check_validity` will trigger the last error
				input.set_custom_validity("");

				// make sure html checks are passed
				if input.check_validity() {
					// we want to display custom error messages if concept wasn't generated
					if let Err(error) = Script::UywiChiffre.from_concept(&value) {
						input.set_custom_validity(&error.to_string());
					}
				}
			},
			// handle clicking on a link to forms
			Event::OpenConcept(concept) => {
				self.view = StateView::Forms {
					page: concept.page(),
					concept,
				};
			},
		}

		return None;
	}
}

/// Build script form.
fn build_script(state: &State) -> Node<SuperEvent> {
	use seed::{attrs, button, class, div, form, option, select};

	return form![
		class!["tr"],
		util::submit_ev(Event::Script),
		div![class!["td"], "Script:"],
		div![
			class!["td"],
			select![
				attrs![At::Name => "input"],
				option![
					attrs![At::Value => 0, At::Selected => (Script::UywiChiffre == state.script).as_at_value()],
					Script::UywiChiffre.to_string()
				],
				option![
					attrs![At::Value => 1, At::Selected => (Script::IpaPeter == state.script).as_at_value()],
					Script::IpaPeter.to_string()
				],
			],
		],
		div![class!["td"], button![attrs![At::Type => "submit", At::Name => "load"], "Load"]]
	];
}

/// Create the table.
fn build_table(state: &State) -> Node<SuperEvent> {
	use seed::{a, attrs, table, td, tr};

	let mut table = table![];

	match state.view {
		// [`StateView::Page`] and [`StateView::Concept`] both show concepts
		StateView::Page { page } | StateView::Concept { page, .. } => {
			// reserve appropriate space for the table
			table.reserve_children(state.length.rows_per_page());

			// only [`StateView::Concept`] has a concept to highlight
			let highlighted_concept = if let StateView::Concept { concept, .. } = state.view {
				Some(concept)
			} else {
				None
			};

			for row in page {
				let mut html_row = tr![];

				// reserve appropriate space for the row
				html_row.reserve_children(state.length.concepts_per_row());

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
						concept.to_string(state.script)
					]);

					html_row.add_child(cell);
				}

				table.add_child(html_row);
			}
		},
		// show forms
		StateView::Forms { concept, .. } => {
			// reserve appropriate space for the table
			table.reserve_children(state.length.stems_per_concept());

			for stem in concept {
				let mut row = tr![];

				// reserve appropriate space for the row
				row.reserve_children(state.length.words_per_stem());

				for form in stem {
					row.add_child(td![form.to_string(state.script)]);
				}

				table.add_child(row);
			}
		},
	}

	return table;
}
