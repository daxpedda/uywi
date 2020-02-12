//! Translates whole sentences.

use crate::{
	home::State as Home,
	util::{self, *},
	Event as SuperEvent, State as SuperState,
};
use seed::prelude::{AsAtValue, At, El, Ev, IndexMap, Node, Orders, UpdateEl};
use uywi::Accent;
use web_sys::{FormData, HtmlFormElement, HtmlTextAreaElement};

/// State of the translator.
#[derive(Default)]
pub struct State;

impl State {
	/// Drawing translator.
	#[allow(clippy::unused_self)]
	pub(crate) fn view(&self) -> Vec<Node<SuperEvent>> {
		use seed::{attrs, button, class, div, form, prelude::ev, textarea};

		return vec![div![
			class!["center"],
			form![
				class!["table"],
				util::submit_ev(Event::Translate),
				div![
					class!["tr"],
					div![
						class!["td"],
						button![attrs![At::Type => "button"], ev(Ev::Click, |_| return Event::Home.into()), "Back to home"],
					]
				],
				div![
					class!["tr"],
					div![
						class!["td"],
						textarea![attrs![
							At::Name => "input",
							At::Required => true.as_at_value(),
							At::AutoComplete => "off",
							At::SpellCheck => "false",
							At::Custom("autocorrect".into()) => "off",
							At::Custom("autocapitalize".into()) => "off",
						],],
					],
				],
				div![
					class!["tr"],
					div![
						class!["td"],
						textarea![attrs![
							At::Name => "output",
							At::Placeholder => "Translated text will be here.",
							At::ReadOnly => true.as_at_value(),
						],],
					],
				],
				div![
					class!["tr"],
					div![class!["td"], button![attrs![At::Type => "submit", At::Name => "translate"], "Translate"]],
				]
			]
		]];
	}

	/// Handling events.
	#[allow(clippy::unused_self)]
	pub(crate) fn update(&mut self, event: Event, _orders: &mut impl Orders<SuperEvent>) -> Option<SuperState> {
		match event {
			Event::Home => return Some(SuperState::Home(Home::default())),
			Event::Translate(form, data) => {
				let mut output = String::new();

				for word in data.pget("input").split_whitespace() {
					match Accent::UywiChiffre.from_string(word) {
						Ok(word) => output.push_str(&word.to_string(Accent::IpaPeter)),
						Err(_) => output.push_str(word),
					}

					output.push(' ');
				}

				form.pget::<HtmlTextAreaElement>("output").set_value(&output);
			},
		};

		return None;
	}
}

/// Translator events.
#[derive(Debug, Clone)]
pub(crate) enum Event {
	/// Go back to the homescreen.
	Home,
	/// Translate!
	Translate(HtmlFormElement, FormData),
}
