//! Home screen.

use crate::{dictionary::State as Dictionary, translator::State as Translator, Event as SuperEvent, State as SuperState};
use seed::prelude::{At, El, Ev, IndexMap, Node, Orders, UpdateEl};

/// State of the homescreen.
#[derive(Default)]
pub struct State;

impl State {
	/// Drawing homescreen.
	#[allow(clippy::unused_self)]
	pub(crate) fn view(&self) -> Vec<Node<SuperEvent>> {
		use seed::{attrs, button, class, div, prelude::ev};

		return vec![div![
			class!["center"],
			div![
				button![
					attrs![At::Type => "button"],
					ev(Ev::Click, |_| return Event::Dictionary.into()),
					"Dictionary"
				],
				button![
					attrs![At::Type => "button"],
					ev(Ev::Click, |_| return Event::Translator.into()),
					"Translator"
				],
			]
		]];
	}

	/// Handling events.
	#[allow(clippy::unused_self)]
	pub(crate) fn update(&mut self, event: Event, _orders: &mut impl Orders<SuperEvent>) -> Option<SuperState> {
		return Some(match event {
			Event::Dictionary => SuperState::Dictionary(Dictionary::default()),
			Event::Translator => SuperState::Translator(Translator::default()),
		});
	}
}

/// Homescreen events.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Event {
	/// Show dictionary.
	Dictionary,
	/// Show translator.
	Translator,
}
