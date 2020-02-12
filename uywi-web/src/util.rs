//! Utility functions.

use seed::{
	prelude::{Ev, Node},
	EventHandler,
};
#[cfg(debug_assertions)]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
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

/// Utility trait for checked extraction from `HtmlFormElement`.
pub trait PHtmlFormElement {
	/// Get element by name and panic if not found or couldn't get converted.
	fn pget<T: JsCast>(&self, name: &str) -> T;
}

impl PHtmlFormElement for HtmlFormElement {
	fn pget<T: JsCast>(&self, name: &str) -> T {
		return self
			.elements()
			.get_with_name(name)
			.expect("no field found")
			.dyn_into::<T>()
			.expect("failed to convert `Element`");
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
pub fn submit_ev<EF: Into<EI>, EI>(handler: impl FnOnce(HtmlFormElement, FormData) -> EF + 'static + Clone) -> EventHandler<EI> {
	use seed::prelude::ev;

	return ev(Ev::Submit, move |event| {
		event.prevent_default();

		let form = event
			.target()
			.expect("event doesn't have a target")
			.dyn_into()
			.expect("target is not a `HtmlFormElement`");

		return handler(
			form,
			FormData::new_with_form(
				event
					.target()
					.expect("event doesn't have a target")
					.dyn_ref()
					.expect("target is not a `HtmlFormElement`"),
			)
			.expect("`FormData` couldn't be made from `HtmlFormElement`"),
		)
		.into();
	});
}

/// Create an event that accepts a closure and passes a `web_sys::HtmlInputElement` with it's value as `String`.
pub fn input_ev<EF: Into<EI>, EI>(handler: impl FnOnce(HtmlInputElement, String) -> EF + 'static + Clone) -> EventHandler<EI> {
	use seed::prelude::ev;

	return ev(Ev::Input, move |event| {
		let input = event
			.target()
			.expect("event doesn't have a target")
			.dyn_into::<HtmlInputElement>()
			.expect("target is not a `HtmlInputElement`");

		let value = input.value();

		return handler(input, value).into();
	});
}

/// Create an event that accepts a closure and passes `value`.
/// Optionally use `web_sys::Event::prevent_default`.
pub fn click_ev<T: Clone + 'static, EF: Into<EI>, EI>(value: T, handler: impl FnOnce(T) -> EF + 'static + Clone) -> EventHandler<EI> {
	use seed::prelude::ev;

	return ev(Ev::Click, move |event| {
		event.prevent_default();

		return handler(value).into();
	});
}

#[cfg(debug_assertions)]
#[wasm_bindgen(inline_js = "export function set_stacktracelimit(limit) { Error.stackTraceLimit = limit; }")]
extern "C" {
	/// Translates to `Error.stackTraceLimit = limit` for increased size of the stacktrace.
	#[allow(clippy::needless_pass_by_value, clippy::missing_docs_in_private_items)]
	pub fn set_stacktracelimit(limit: f32);
}
