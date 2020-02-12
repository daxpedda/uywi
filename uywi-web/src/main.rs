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

mod dictionary;
mod home;
mod translator;
mod util;

use dictionary::{Event as DictionaryEvent, State as Dictionary};
use home::{Event as HomeEvent, State as Home};
use seed::{
	app::builder::before_mount::BeforeMount,
	prelude::{Orders, View},
	App,
};
use translator::{Event as TranslatorEvent, State as Translator};

/// Data we save for the app.
enum State {
	/// Showing home screen.
	Home(Home),
	/// Showing dictionary.
	Dictionary(Dictionary),
	/// Showing translator.
	Translator(Translator),
}

impl Default for State {
	fn default() -> Self {
		return Self::Home(Home);
	}
}

/// Drawing app.
fn view(state: &State) -> impl View<Event> {
	return match state {
		State::Home(state) => state.view(),
		State::Dictionary(state) => state.view(),
		State::Translator(state) => state.view(),
	};
}

/// Events that are sent by the app.
#[derive(Debug, Clone)]
enum Event {
	/// Home events.
	Home(HomeEvent),
	/// Dictionary events.
	Dictionary(DictionaryEvent),
	/// Translator events.
	Translator(TranslatorEvent),
}

impl From<HomeEvent> for Event {
	fn from(event: HomeEvent) -> Self {
		return Self::Home(event);
	}
}

impl From<DictionaryEvent> for Event {
	fn from(event: DictionaryEvent) -> Self {
		return Self::Dictionary(event);
	}
}

impl From<TranslatorEvent> for Event {
	fn from(event: TranslatorEvent) -> Self {
		return Self::Translator(event);
	}
}

/// Handling events.
fn update(event: Event, state: &mut State, orders: &mut impl Orders<Event>) {
	let switch = match event {
		Event::Home(event) => {
			if let State::Home(home) = state {
				home.update(event, orders)
			} else {
				unreachable!("reached different state then event")
			}
		},
		Event::Dictionary(event) => {
			if let State::Dictionary(state) = state {
				state.update(event, orders)
			} else {
				unreachable!("reached different state then event")
			}
		},
		Event::Translator(event) => {
			if let State::Translator(state) = state {
				state.update(event, orders)
			} else {
				unreachable!("reached different state then event")
			}
		},
	};

	if let Some(switch) = switch {
		*state = switch;
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

	App::builder(update, view)
		.before_mount(|_| return BeforeMount::new().mount_point(seed::util::body()))
		.build_and_start();
}
