//! # Googology
//! Googlogy is the [study and nomenclature of large numbers.](https://googology.wikia.org/wiki/Googology)
//! This crate is focused on the latter component of that description,
//! and provides utilities for converting numbers of arbitrarily large size
//! to some natural-language description.
//! 
//! Currently, this crate supports two systems for naming large numbers. 
//! 
//! The first is the [Conway-Wechsler]((http://www.mrob.com/pub/math/largenum.html#conway-wechsler))
//! system, which uses names that may be familiar to English speakers, such as
//! million, billion, and so on... By chaining latin prefixes together (such as in
//! the number "one millitrillion"), it is possible it is possible to use this
//! system to generate a name for any number, provided one has enough memory to
//! store both the digits in the input, as well as the string output.
//! 
//! This system supports three different "scale" parameters:
//! * `Scale::Short` uses a modern English naming convention where each new "illion"
//! is scaled by powers of 1,000. The value of 10^9 is called `"one billion"`.
//! * `Scale::LongBritish` uses an older convention used in the UK prior to 1974.
//! Each new "illion" is scaled by powers of 1,000,000, and powers of 1,000 that
//! lie in between "illions" are prefixed with "thousand". Thus, the value of 10^9
//! is called `"one thousand million"`.
//! * `Scale::LongPeletier` uses a naming convention still in use in many European
//! languages. Similar to `Scale::LongBritish`, "illions" are scaled by powers of
//! 1,000,000. However, instead of prefixing the in betweens with "thousand", they
//! are instead suffixed with "ard" instead of "on". Thus, the value of 10^9 is
//! called `"one milliard"`. 
//! 
//! An alternative system called the Knuth-Yllion system is also provided. Here,
//! rather than scaling by powers of 1,000 or powers of 1,000,000, the scaling is
//! instead exponential. A new name is given for each n in 10^(2^n). For example,
//! 10^2 is one hundred, 10^4 is one myriad, and 10^8 is one myllion. For values
//! in between, we describe an "yllion" number with those of lesser magnitude. For
//! example, 10^14 would be called "one hundred myriad myllion".
//! 
//! Two functions are provided in each module:
//! * `full_name` gives a name to any arbitrary number, given a base-10 string
//! representation of its digits.
//! * `power_of_ten` gives a name to a power of ten. This can be useful for numbers
//! that may be so large that storing them in memory would be impossible or
//! otherwise impractical.


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
