//! UYWI pages.

use crate::*;
use std::fmt::{self, Display, Formatter};

/// All pages, only used for iteration.
#[derive(Clone, Debug)]
pub struct Pages {
	/// Concept length.
	length: Length,
	/// Current page index, saved for iteration.
	page_index: usize,
}

impl Pages {
	/// Build [`Pages`] from [`Length`].
	#[must_use]
	pub const fn new(length: Length) -> Self {
		return Self { length, page_index: 0 };
	}
}

impl Iterator for Pages {
	type Item = Page;

	fn next(&mut self) -> Option<Self::Item> {
		// check if we reached the last page
		if self.page_index < self.length.num_of_pages() {
			let page = Some(Page::from_index(self.page_index, self.length).expect("failed to build page"));

			// increment page by one
			self.page_index = self.page_index.padd(1);

			return page;
		} else {
			return None;
		}
	}
}

/// A page, used for type checking.
#[derive(Clone, Copy, Debug)]
pub struct Page {
	/// Page index.
	index: usize,
	/// Concept length.
	length: Length,
}

impl Page {
	/// Build [`Page`] from page index.
	pub fn from_index(index: usize, length: Length) -> Result<Self> {
		if index >= length.num_of_pages() {
			return Err(Error::PageIndexInvalid);
		}

		return Ok(Self { index, length });
	}

	/// Build [`Page`] from page string.
	pub fn from_str(index: &str, length: Length) -> Result<Self> {
		let index: usize = if let Ok(index) = index.parse() {
			index
		} else {
			return Err(Error::PageStringInvalid);
		};

		// in index form a page is always `- 1` to the string
		return Self::from_index(index.psub(1), length);
	}

	/// Get index.
	#[must_use]
	pub const fn index(self) -> usize {
		return self.index;
	}

	/// Get length.
	#[must_use]
	pub const fn length(self) -> Length {
		return self.length;
	}
}

impl IntoIterator for Page {
	type Item = Row;
	type IntoIter = Rows;

	#[must_use]
	fn into_iter(self) -> Self::IntoIter {
		return Rows::new(self);
	}
}

impl Display for Page {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		// in string form a page is always `+ 1` to the index
		return write!(formatter, "{}", self.index.padd(1));
	}
}
