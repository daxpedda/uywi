//! Translates whole sentences.

use crate::{
	home::State as Home,
	util::{self, *},
	Event as SuperEvent, State as SuperState,
};
use seed::prelude::{AsAtValue, At, El, Ev, IndexMap, Node, Orders, St, UpdateEl};
use uywi::Script;
use web_sys::{FormData, HtmlFormElement, HtmlTextAreaElement};

/// State of the translator.
#[derive(Default)]
pub struct State;

impl State {
	/// Drawing translator.
	#[allow(clippy::unused_self)]
	pub(crate) fn view(&self) -> Vec<Node<SuperEvent>> {
		use seed::{attrs, button, class, div, form, prelude::ev, style, textarea};

		return vec![form![
			class!["center"],
			style![St::GridTemplateRows => "min-content auto auto min-content", St::GridRowGap => "1em"],
			util::submit_ev(Event::Translate),
			div![button![
				attrs![At::Type => "button"],
				ev(Ev::Click, |_| return Event::Home.into()),
				"Back to home"
			]],
			div![
				style![St::JustifySelf => "stretch", St::AlignSelf => "stretch"],
				textarea![attrs![
					At::Name => "input",
					At::Required => true.as_at_value(),
					At::AutoComplete => "off",
					At::SpellCheck => "false",
					At::Custom("autocorrect".into()) => "off",
					At::Custom("autocapitalize".into()) => "off",
				]]
			],
			div![
				style![St::JustifySelf => "stretch", St::AlignSelf => "stretch"],
				textarea![attrs![
					At::Name => "output",
					At::Placeholder => "Translated text will be here.",
					At::ReadOnly => true.as_at_value(),
				]]
			],
			div![button![attrs![At::Type => "submit", At::Name => "translate"], "Translate"]]
		]];
	}

	/// Handling events.
	#[allow(clippy::unused_self)]
	pub(crate) fn update(&mut self, event: Event, _orders: &mut impl Orders<SuperEvent>) -> Option<SuperState> {
		match event {
			Event::Home => return Some(SuperState::Home(Home::default())),
			Event::Translate(form, data) => {
				let mut output = String::new();
				let mut word = String::new();

				let handle_word = |output: &mut String, word: &mut String| {
					if !word.is_empty() {
						if let Ok(word) = Script::UywiChiffre.from_str(word) {
							output.push_str(&word.to_string(Script::IpaPeter));
						} else {
							output.push_str(word);
						}

						word.clear();
					}
				};

				for char in data.pget("input").chars() {
					if char.is_whitespace() {
						handle_word(&mut output, &mut word);
						output.push(char);
					} else {
						word.push(char);
					}
				}

				handle_word(&mut output, &mut word);

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
