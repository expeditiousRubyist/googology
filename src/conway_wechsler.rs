//! Methods for the Conway-Wechsler large number naming system.
//! A proper description of how this system works can be found on
//! [this website](http://www.mrob.com/pub/math/largenum.html#conway-wechsler).
//! 
//! We add to this system just slightly by allowing the user to choose between
//! the short scale (thousand, million, billion, trillion...), which is the
//! standard in modern English, and the long scale (thousand, million, milliard,
//! billion...), which is more common in European languages, and which used to
//! be the standard for British English.

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
fn zillion_number(num: usize, short: bool) -> Result<String, ParseError> {
	if num == 0 { return Ok(String::from("")); }
	if num == 1 { return Ok(String::from("thousand")); }
	
	let mut name   = String::from("");
	let mut power  = num - 1;
	let mut suffix = "on";

	// Adjust for long scale
	if !short {
		if num % 2 != 0 { suffix = "ard"; }
		power = ((num + 2) / 2) - 1;
	}

	// Prefixes technically added in reverse order here.
	// e.g. in millinillion, first add "nilli", then "milli", then "on".
	while power > 0 {
		let prefix = zillion_prefix(power % 1000)?;
		name.insert_str(0, prefix.as_str());
		power /= 1000;
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
/// * `short` - A boolean to determine whether or not the short scale should
/// be used. For reference, the represented by `10^9` is called "one billion"
/// using the short scale. When `short` is set to false, the long scale is used
/// instead, and this number would be called "one milliard".
/// 
/// # Example
/// 
/// ```
/// use googology::conway_wechsler::full_name;
/// let milliard = full_name("19000000042", false).unwrap();
/// let billion = full_name("19000000042", true).unwrap();
/// assert_eq!("nineteen milliard forty two", milliard.as_str());
/// assert_eq!("nineteen billion forty two", billion.as_str());
/// ```
pub fn full_name(digits: &str, short: bool) -> Result<String, ParseError> {
	// Sanity checks. We want the string to be entirely digits, and we want
	// to handle the case of leading zeroes. If all digits are zero, we want
	// to just return the string "zero", and otherwise process from the
	// first nonzero character.
	let first_nonzero = is_all_digits(digits)
		.then(|| digits)
		.ok_or(ParseError::InvalidDigit)
		.map(|d| d.find(|c| c != '0'))?;

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
		let zillion = zillion_number(remaining / 3, short)?;

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
		let zillion = zillion_number(remaining / 3 - 1, short)?;

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
/// * `short` - A boolean to determine whether or not the short scale should
/// be used. For reference, the represented by `10^9` is called "one billion"
/// using the short scale. When `short` is set to false, the long scale is used
/// instead, and this number would be called "one milliard".
/// 
/// # Example
///
/// ```
/// use googology::conway_wechsler::power_of_ten;
/// let milliard = power_of_ten("9", false).unwrap();
/// let billion = power_of_ten("9", true).unwrap();
/// assert_eq!("one milliard", milliard.as_str());
/// assert_eq!("one billion", billion.as_str());
/// ```
pub fn power_of_ten(digits: &str, short: bool) -> Result<String, ParseError> {
	// Sanity check. We want to convert our input string into a Bignum.
	// The num_bigint crate doesn't quite allow us to know the cause of
	// error, but from what we can tell, it's either an invalid digit or
	// an empty string. So we'll make this clear in our own error.
	let mut power = is_all_digits(digits)
		.then(|| digits)
		.ok_or(ParseError::InvalidDigit)
		.and_then(|d| BigUint::from_str(d).map_err(|_| ParseError::Empty))?;

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
	let loc = output.len(); // Location to insert prefixes at

	// Adjust for long scale if necessary
	let mut suffix = "on";
	if !short {
		suffix = if (&power % 2u32).is_zero() { "ard" } else { "on" };
		power += 3u32;
		power /= 2u32;
		power -= 1u32;
	}

	// Add prefixes in reverse order because we are stupid and inefficient.
	while !power.is_zero() {
		let prefix = (&power % 1000u32)
			.to_usize()
			.ok_or(ParseError::InternalError)
			.and_then(zillion_prefix)?;

		output.insert_str(loc, prefix.as_str());
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
		let zero_ss = full_name("0", true)?;
		let zero_ls = full_name("0", false)?;
		assert_eq!("zero", zero_ss.as_str());
		assert_eq!("zero", zero_ls.as_str());
		Ok(())	
	}

	#[test]
	fn small_numbers() -> Result<(), ParseError> {
		let twelve_ss = full_name("12", true)?;
		let twelve_ls = full_name("12", false)?;
		assert_eq!("twelve", twelve_ss.as_str());
		assert_eq!("twelve", twelve_ls.as_str());
		Ok(())
	}

	#[test]
	fn large_numbers() -> Result<(), ParseError> {
		let billion = full_name("1000000000", true)?;
		let milliard = full_name("1000000000", false)?;
		assert_eq!("one milliard", milliard.as_str());
		assert_eq!("one billion", billion.as_str());
		Ok(())
	}

	#[test]
	fn large_powers() -> Result<(), ParseError> {
		let googol_ss = power_of_ten("100", true)?;
		let googol_ls = power_of_ten("100", false)?;
		assert_eq!("ten duotrigintillion", googol_ss.as_str());
		assert_eq!("ten sedecilliard", googol_ls.as_str());
		Ok(())
	}
}