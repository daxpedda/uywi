//! Utility functions.

use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use std::{convert::TryInto, fmt};
use unicode_segmentation::UnicodeSegmentation;

/// Convert between number types with panicking.
pub trait PConvert<T> {
	/// Convert `Self` to `T`. Panics on overflows.
	fn pinto(self) -> T;

	/// Convert to `Self` from `T`. Panics on overflows.
	fn pfrom(from: T) -> Self;
}

impl<T, S> PConvert<S> for T
where
	T: TryInto<S>,
	<T as TryInto<S>>::Error: fmt::Debug,
	S: TryInto<Self>,
	<S as TryInto<Self>>::Error: fmt::Debug,
{
	fn pinto(self) -> S {
		return self.try_into().expect("failed to convert");
	}

	fn pfrom(from: S) -> Self {
		return from.try_into().expect("failed to convert");
	}
}

/// Utility trait for checked addition.
pub trait PAdd {
	/// Adds by value and panics on overflow.
	fn padd(self, value: Self) -> Self;
}

impl<T> PAdd for T
where
	T: CheckedAdd,
{
	fn padd(self, value: Self) -> Self {
		return self.checked_add(&value).expect("integer overflow");
	}
}

/// Utility trait for checked subtraction.
pub trait PSub {
	/// Subtracts by value and panics on overflow.
	fn psub(self, value: Self) -> Self;
}

impl<T> PSub for T
where
	T: CheckedSub,
{
	fn psub(self, value: Self) -> Self {
		return self.checked_sub(&value).expect("integer overflow");
	}
}

/// Utility trait for checked multiplication.
pub trait PMul {
	/// Multiplies by value and panics on overflow.
	fn pmul(self, value: Self) -> Self;
}

impl<T> PMul for T
where
	T: CheckedMul,
{
	fn pmul(self, value: Self) -> Self {
		return self.checked_mul(&value).expect("integer overflow");
	}
}

/// Utility trait for checked division.
pub trait PDiv {
	/// Divides by value and panics on overflow.
	fn pdiv(self, value: Self) -> Self;
}

impl<T> PDiv for T
where
	T: CheckedDiv,
{
	fn pdiv(self, value: Self) -> Self {
		return self.checked_div(&value).expect("integer overflow");
	}
}

/// Convenience functions for strings to seperate between chars and graphemes.
pub trait GraphemeIdentity {
	/// Get length.
	fn grapheme_len(&self) -> usize;

	/// Get specific grapheme.
	fn grapheme_nth(&'_ self, index: usize) -> Option<&'_ str>;
}

impl GraphemeIdentity for str {
	fn grapheme_len(&self) -> usize {
		return self.graphemes(true).count();
	}

	fn grapheme_nth(&'_ self, index: usize) -> Option<&'_ str> {
		return self.graphemes(true).nth(index);
	}
}
