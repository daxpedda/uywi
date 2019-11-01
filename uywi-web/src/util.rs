//! Utility functions.

use seed::{
	events::Listener,
	prelude::{Ev, Node},
};
#[cfg(debug_assertions)]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlInputElement};
use wee_alloc::WeeAlloc;

/// WASM optimized allocator.
#[global_allocator]
static _ALLOCATOR: WeeAlloc = WeeAlloc::INIT;

/// Utility trait for checked extraction from `FromData`.
pub trait PFormData {
	/// Get by name and panic if not found or couldn't get converted.
	fn pget(&self, name: &str) -> String;
}

impl PFormData for FormData {
	fn pget(&self, name: &str) -> String {
		return self
			.get(name)
			.as_string()
			.expect("extracted form field couldn't be converted into a string");
	}
}

/// Utility trait for reserving space for children in a `Node`.
pub trait ReserveChildren {
	/// Reserve space in the children container of the element.
	fn reserve_children(&mut self, size: usize) -> &mut Self;
}

impl<T> ReserveChildren for Node<T> {
	fn reserve_children(&mut self, size: usize) -> &mut Self {
		if let Self::Element(el) = self {
			el.children.reserve_exact(size);
		}

		return self;
	}
}

/// Create an event that accepts a closure and passes a `web_sys::FormData`.
/// Optionally use `web_sys::Event::prevent_default`.
pub fn submit_ev<T>(handler: impl FnOnce(FormData) -> T + 'static + Clone) -> Listener<T> {
	use seed::prelude::raw_ev;

	return raw_ev(Ev::Submit, move |event| {
		event.prevent_default();

		return handler(
			FormData::new_with_form(
				event
					.target()
					.expect("event doesn't have a target")
					.dyn_ref()
					.expect("target is not a `HtmlFormElement`"),
			)
			.expect("`FormData` couldn't be made from `HtmlFormElement`"),
		);
	});
}

/// Create an event that accepts a closure and passes a `web_sys::HtmlInputElement` with it's value as `String`.
pub fn input_ev<T>(handler: impl FnOnce(HtmlInputElement, String) -> T + 'static + Clone) -> Listener<T> {
	use seed::prelude::raw_ev;

	return raw_ev(Ev::Input, move |event| {
		let input = event
			.target()
			.expect("event doesn't have a target")
			.dyn_into::<HtmlInputElement>()
			.expect("target is not a `HtmlInputElement`");

		let value = input.value();

		return handler(input, value);
	});
}

/// Create an event that accepts a closure and passes `value`.
/// Optionally use `web_sys::Event::prevent_default`.
pub fn click_ev<T: Clone + 'static, E>(value: T, handler: impl FnOnce(T) -> E + 'static + Clone) -> Listener<E> {
	use seed::prelude::raw_ev;

	return raw_ev(Ev::Click, move |event| {
		event.prevent_default();

		return handler(value);
	});
}

#[cfg(debug_assertions)]
#[wasm_bindgen(inline_js = "export function set_stacktracelimit(limit) { Error.stackTraceLimit = limit; }")]
extern "C" {
	/// Translates to `Error.stackTraceLimit = limit` for increased size of the stacktrace.
	#[allow(clippy::needless_pass_by_value, clippy::missing_docs_in_private_items)]
	pub fn set_stacktracelimit(limit: f32);
}
