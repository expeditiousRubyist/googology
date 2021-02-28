//! # Googology
//! Googlogy is the [study and nomenclature of large numbers.](https://googology.wikia.org/wiki/Googology)
//! This crate is focused on the latter component of that description,
//! and provides utilities for converting numbers of arbitrarily large size
//! to some English-language description.
//! 
//! Currently, this crate only supports the Conway-Wechsler system for
//! naming numbers, however other systems may be added in the future.

mod common;
pub mod conway_wechsler;
pub mod knuth_yllion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
	/// Input was the empty string.
	Empty,
	/// Input is too large to be given a name by knuth_yllion.
	InputTooLarge,
	/// The parser entered some sort of invalid state.
	/// If this error is returned, there is a bug in the googology crate.
	InternalError,
	/// Input contains some digits other than 0-9.
	InvalidDigit,
}
