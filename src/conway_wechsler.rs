//! Methods for the Conway-Wechsler large number naming system.
//! A proper description of how this system works can be found on
//! [this website](http://www.mrob.com/pub/math/largenum.html#conway-wechsler).
//! 
//! We add to this system just slightly by allowing the user to choose between
//! three different scales. The short scale (thousand, million, billion,
//! trillion, ...) is the standard in modern English, while the long scale
//! (thousand, million, milliard, billion, ...) is more common in European
//! languages. It is also possible to use a variant of the long scale previously
//! used in the UK before switching to the short scale, where instead of using
//! milliard to refer to the value 10^9, the term "one thousand million" is used
//! instead.

extern crate num_traits;
extern crate num_bigint;

use std::str::FromStr;
use num_traits::cast::ToPrimitive;
use num_traits::identities::Zero;
use num_traits::identities::One;
use num_bigint::BigUint;

use crate::common::{
	is_all_digits,
	num_from_slice,
	latin_prefix,
	myriad_number
};

use crate::ParseError;

/// A parameter for Conway-Wechsler functions which indicates how number names
/// change every power of 1000.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
	/// The scale typically used in modern English-speaking nations.
	/// 10^9 will be called one billion.
	Short,
	/// The scale formerly used in the UK, and based on Chuquet's
	/// scheme for naming numbers. 10^9 will be called one thousand million.
	LongBritish,
	/// The scale currently in use by many European languages, most
	/// notably French. 10^9 will be called one milliard.
	LongPeletier,
}


// Create a name for a single 3 digit zillion number, ending in -illi.
// Value for zero is "nilli", for use in chained zillion numbers.
// Values above 999 will panic.
fn zillion_prefix(num: usize) -> Result<String, ParseError> {
	let mut name = latin_prefix(num)?;
	name.push_str("illi");
	Ok(name)
}

// Create a name for an arbitrary power of 1000.
// Value for zero is the empty string.
// Value for one is "thousand".
// Value for anything greater will involve repeated application of the
// zillion_prefix function, to create a number ending in "illion",
// or "ard" depending on whether or not we are using the long scale.
fn zillion_number(num: usize, scale: Scale) -> Result<String, ParseError> {
	if num == 0 { return Ok(String::from("")); }
	if num == 1 { return Ok(String::from("thousand")); }

	// Create adjustments to name for long scale.
	let (prefix, suffix) = match (scale, num % 2) {
		(Scale::LongBritish, 1)  => ("thousand ", "on"),
		(Scale::LongPeletier, 1) => ("", "ard"),
		(_, _) => ("", "on"),
	};

	let mut power = match scale {
		Scale::Short => num - 1,
		_ => ((num + 2) / 2) - 1,
	};

	let mut name = String::from(prefix);

	// Zillion prefixes added in reverse order here.
	// e.g. in millinillion, first add "nilli", then "milli", then "on".
	let mut zillions = Vec::with_capacity(7);
	while power > 0 {
		let zillion = zillion_prefix(power % 1000)?;
		zillions.push(zillion);
		power /= 1000;
	}

	for z in zillions.iter().rev() {
		name.push_str(z.as_str());
	}

	name.push_str(suffix);
	Ok(name)
}

/// Gives a full length name for a number represented by an arbitrary sequence
/// of digits.
///
/// # Arguments
/// 
/// * `digits` - A string slice that holds a representation of the number
/// using only the digits 0-9. If any other character is present, this function
/// will return an Err.
/// * `scale` - An enumerated value to determine which scale should
/// be used. Short scales use a new "-illion" name for every power of 1000,
/// while long scales use a new "-illion" name for every power of 1000000.
/// 
/// # Example
/// 
/// ```
/// use googology::conway_wechsler::{Scale, full_name};
/// let milliard = full_name("19000000042", Scale::LongPeletier).unwrap();
/// let billion = full_name("19000000042", Scale::Short).unwrap();
/// assert_eq!("nineteen milliard forty two", milliard.as_str());
/// assert_eq!("nineteen billion forty two", billion.as_str());
/// ```
pub fn full_name(digits: &str, scale: Scale) -> Result<String, ParseError> {
	// Sanity checks. We want the string to be entirely digits, and we want
	// to handle the case of leading zeroes. If all digits are zero, we want
	// to just return the string "zero", and otherwise process from the
	// first nonzero character.
	let first_nonzero = is_all_digits(digits)
		.then(|| digits)
		.ok_or(ParseError::InvalidDigit)
		.and_then(|d|
			if d.is_empty() { Err(ParseError::Empty) }
			else { Ok(d.find(|c| c != '0')) }
		)?;

	let (mut i, mut output) = first_nonzero.map_or_else(
		|| (0, String::from("zero")),
		|idx| (idx, String::from(""))
	);

	if !output.is_empty() { return Ok(output); }

	// Determine how many digits to process, and how many digits are in the
	// first zillion (e.g. 2 in the case of 12 tredecillion).
	let mut remaining = digits.len() - i;
	let first = remaining % 3;

	if first > 0 {
		let num     = num_from_slice(digits, i, first);
		let leading = myriad_number(num)?;
		let zillion = zillion_number(remaining / 3, scale)?;

		output.push_str(leading.as_str());
		if !zillion.is_empty() {
			output.push(' ');
			output.push_str(zillion.as_str());
		}

		remaining -= first;
		i += first;
	}

	// Handle the rest of the digits in chunks of three at a time.
	while remaining > 0 {
		let num     = num_from_slice(digits, i, 3);
		let leading = myriad_number(num)?;
		let zillion = zillion_number(remaining / 3 - 1, scale)?;

		if !leading.is_empty() {
			if !output.is_empty() { output.push(' '); }

			output.push_str(leading.as_str());
			if !zillion.is_empty() {
				output.push(' ');
				output.push_str(zillion.as_str());
			}
		}

		i += 3;
		remaining -= 3;
	}

	Ok(output)
}

/// Gives a name for a number representing a power of ten.
/// This function is equivalent to using `full_name` with a one followed by
/// as many zeroes as would be indicated the number described by `digits`.
///
/// # Arguments
/// 
/// * `digits` - A string slice that holds a representation of the number
/// using only the digits 0-9. If any other character is present, this function
/// will return an Err.
/// * `scale` - An enumerated value to determine which scale should
/// be used. Short scales use a new "-illion" name for every power of 1000,
/// while long scales use a new "-illion" name for every power of 1000000.
/// 
/// # Example
///
/// ```
/// use googology::conway_wechsler::{Scale, power_of_ten};
/// let thousand_million = power_of_ten("9", Scale::LongBritish).unwrap();
/// let billion = power_of_ten("9", Scale::Short).unwrap();
/// assert_eq!("one thousand million", thousand_million.as_str());
/// assert_eq!("one billion", billion.as_str());
/// ```
pub fn power_of_ten(digits: &str, scale: Scale) -> Result<String, ParseError> {
	// Sanity check. We want to convert our input string into a Bignum.
	// The num_bigint crate doesn't quite allow us to know the cause of
	// error, but from what we can tell, it's either an invalid digit or
	// an empty string. So we'll make this clear in our own error.
	let mut power = is_all_digits(digits)
		.then(|| digits)
		.ok_or(ParseError::InvalidDigit)
		.and_then(|d| 
			if d.is_empty() { Err(ParseError::Empty) }
			else { Ok(d) }
		)
		.and_then(|d| BigUint::from_str(d).map_err(|_| ParseError::InternalError))?;

	// Get the leading word (e.g. "ten" in "ten million")
	let s = (&power % 3u32)
		.to_u32()
		.map(|m| match m { 
			0 => "one",
			1 => "ten", 
			2 => "one hundred",
			_ => "" 
		})
		.unwrap_or("");

	let mut output = String::from(s);

	// Convert into power of one thousand
	// We may return early for edge cases.
	power /= 3u32;
	if power.is_zero() { return Ok(output); }
	if power.is_one() {
		output.push_str(" thousand");
		return Ok(output);
	}

	// Compute zillion number.
	power -= 1u32;
	output.push_str(" ");

	// Adjust for long scale if necessary
	let (prefix, suffix) = match (scale, (&power % 2u32).is_zero()) {
		(Scale::Short, _) | (_, false) => ("", "on"),
		(Scale::LongBritish, true)     => ("thousand ", "on"),
		(Scale::LongPeletier, true)    => ("", "ard"),
	};

	if scale != Scale::Short {
		power += 3u32;
		power /= 2u32;
		power -= 1u32;
	}

	output.push_str(prefix);
	let loc = output.len(); // Location to insert prefixes at

	// Add zillions in reverse order because we are stupid and inefficient.
	while !power.is_zero() {
		let zillion = (&power % 1000u32)
			.to_usize()
			.ok_or(ParseError::InternalError)
			.and_then(zillion_prefix)?;

		output.insert_str(loc, zillion.as_str());
		power /= 1000u32;
	}

	output.push_str(suffix);
	Ok(output)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn very_small_numbers() -> Result<(), ParseError> {
		let zero_ss = full_name("0", Scale::Short)?;
		let zero_lp = full_name("0", Scale::LongPeletier)?;
		assert_eq!("zero", zero_ss.as_str());
		assert_eq!("zero", zero_lp.as_str());
		Ok(())	
	}

	#[test]
	fn small_numbers() -> Result<(), ParseError> {
		let twelve_ss = full_name("12", Scale::Short)?;
		let twelve_lb = full_name("12", Scale::LongBritish)?;
		assert_eq!("twelve", twelve_ss.as_str());
		assert_eq!("twelve", twelve_lb.as_str());
		Ok(())
	}

	#[test]
	fn large_numbers() -> Result<(), ParseError> {
		let billion = full_name("1000000000", Scale::Short)?;
		let milliard = full_name("1000000000", Scale::LongPeletier)?;
		let thousand_million = full_name("1000000000", Scale::LongBritish)?;
		assert_eq!("one billion", billion.as_str());
		assert_eq!("one milliard", milliard.as_str());
		assert_eq!("one thousand million", thousand_million.as_str());
		Ok(())
	}

	#[test]
	fn large_powers() -> Result<(), ParseError> {
		let googol_ss = power_of_ten("100", Scale::Short)?;
		let googol_lb = power_of_ten("100", Scale::LongBritish)?;
		let googol_lp = power_of_ten("100", Scale::LongPeletier)?;
		assert_eq!("ten duotrigintillion", googol_ss.as_str());
		assert_eq!("ten thousand sedecillion", googol_lb.as_str());
		assert_eq!("ten sedecilliard", googol_lp.as_str());
		Ok(())
	}
}