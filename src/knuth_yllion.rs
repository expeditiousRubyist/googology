//! Methods for the Knuth -yllion large number naming system.
//! This is a myriad number system based on Donald Knuth's essay,
//! "Supernatural Numbers", which was published in the 1981 book,
//! "The Mathematical Gardner". The basic concept behind this system is that
//! each new "yllion" is the square of the last (i.e. where one myllion is equal
//! to 10^8, one byllion is 10^16 instead of 10^12 if we were multipling by
//! constant factors like in an -illion system).
//! 
//! Knuth's essay only provides names for yllion numbers up to one vigintyllion,
//! which is 10^4194304 (or 10^(2^22)). Due to this crate exporting a function
//! called `power_of_ten`, which should provide a proper name for any power of
//! ten whose exponent can be expressed as a string, we attempt to extend this
//! system. Using the same latin naming scheme in Conway-Weschler without adding
//! extra -ylli components, we can reach up to 10^(2^1001) with the number
//! "one novenonagintanongentyllion".
//! 
//! Beyond this number, we will use Knuth's system which prefixes the word
//! "latin" upon some yllion, in the following fashion:
//! 
//! For 10^(2^(n+2)), call this: "latin{word for n with spaces removed}yllion"
//! 
//! Thus, 10^1002 will be "latintenhundredyllion" and 10^10002 will be called
//! "latinmyriadyllion", and so on.
//! 
//! Using the `full_name` function, however, will not require any significant
//! level of creativity, as a 64-bit system cannot store a string larger than
//! 2^64 bytes long, and in practice, this is much smaller. Accordingly, the
//! largest named number such a system could theoretically output would be on
//! the scale of "one duosexagintyllion" (10^(2^64)). If we were on a RISC-V
//! 128-bit system with a maximum amount of RAM, the largest named number would
//! be "one sesviginticentyllion" (10^(2^128)).

extern crate num_traits;
extern crate num_bigint;

use std::str::FromStr;
use num_traits::cast::ToPrimitive;
use num_traits::identities::Zero;
use num_bigint::BigUint;

use crate::common::{
	is_all_digits,
	num_from_slice,
	latin_prefix,
	myriad_number
};

use crate::ParseError;

// Create a name for an arbitrary grouping of four digits.
// This function's behavior should not be considered perfectly equivalent to the
// zillion_number function on the conway_wechsler module, because it is not
// bijective. The number 12,0000,0042,0000 is given the full_name of
// "twelve myriad myllion forty two myriad", indicating that the word "myriad"
// is intended to be returned both for the grouping containing 42, as well as
// the grouping containing 12.
//
// Note: This function also returns an integer to be compared against the
// last_largest value in the full_name function.
fn zyllion_number(num: usize) -> Result<(String, usize), ParseError> {
	// The last grouping has no qualifier,
	// and every other grouping is just "myriad".
	if num == 0     { return Ok((String::from(""), 0)); }
	if num % 2 == 1 { return Ok((String::from("myriad"), 1)); }

	// For the rest, we want to find the greatest power of 2 that we're a
	// multiple of, convert it into a latin prefix, and add "yllion".
	// Note that the greatest power of two should be in the range [1,63]
	// by necessity, since num is an even-valued usize.
	let mut name = String::from("");
	let greatest_power_of_two = num.trailing_zeros() as usize;
	let prefix = latin_prefix(greatest_power_of_two)?;

	name.push_str(prefix.as_str());
	name.push_str("yllion");
	Ok((name, greatest_power_of_two + 1))
}

/// Gives a full length name for a number represented by an arbitrary sequence
/// of digits.
///
/// # Arguments
/// 
/// * `digits` - A string slice that holds a representation of the number
/// using only the digits 0-9. If any other character is present, this function
/// will return an Err.
/// 
/// # Example
/// 
/// ```
/// use googology::knuth_yllion::full_name;
/// let name = full_name("1200426208").unwrap();
/// let expected = "twelve myllion forty two myriad sixty two hundred eight";
/// assert_eq!(name.as_str(), expected);
/// ```
pub fn full_name(digits: &str) -> Result<String, ParseError> {
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

	// Because each term zyllion term describes the quantity of the next
	// largest term (i.e. one myriad myllion), we keep track of the most
	// recent largest term we've outputted.
	let mut last_largest : usize = 0;

	// Determine number of digits to process, and how many digits are in
	// the first grouping.
	let mut remaining = digits.len() - i;
	let first = remaining % 4;

	if first > 0 {
		let num     = num_from_slice(digits, i, first);
		let leading = myriad_number(num)?;
		let (zyllion, largest) = zyllion_number(remaining / 4)?;

		output.push_str(leading.as_str());
		if !zyllion.is_empty() {
			output.push(' ');
			output.push_str(zyllion.as_str());
			last_largest = largest;
		}

		remaining -= first;
		i += first;
	}

	// Handle the rest of the digits in chunks of four at a time.
	while remaining > 0 {
		let num     = num_from_slice(digits, i, 4);
		let leading = myriad_number(num)?;
		let (zyllion, largest) = zyllion_number((remaining - 1) / 4)?;

		if !leading.is_empty() {
			if !output.is_empty() { output.push(' '); }

			output.push_str(leading.as_str());
			if !zyllion.is_empty() {
				output.push(' ');
				output.push_str(zyllion.as_str());
				last_largest = largest;
			}
		}

		// This condition does not trigger if we wrote a zyllion in the
		// above block of code. Instead, it means that we have a group
		// of all zeroes, but should be writing a zyllion that is larger
		// than the last one that we wrote.
		if largest > last_largest {
			output.push(' ');
			output.push_str(zyllion.as_str());
			last_largest = largest;
		}

		i += 4;
		remaining -= 4;
	}

	Ok(output)
}

/// Gives a name for a number representing a power of ten.
/// This function is equivalent to using `full_name` with a one followed by
/// as many zeroes as would be indicated the number described by `digits`.
/// Due to the exponential nature of Knuth's Yllion system, however, this
/// function may output yllion names that could not by outputted by any input to
/// the `full_name` function. Unfortunately, there are still some inputs too
/// large for this function to handle at the current time, so an Err may still
/// be returned
///
/// # Arguments
/// 
/// * `digits` - A string slice that holds a representation of the number
/// using only the digits 0-9. If any other character is present, this function
/// will return an Err.
/// 
/// # Example
///
/// ```
/// use googology::knuth_yllion::power_of_ten;
/// let one_hundred_myllion = power_of_ten("10").unwrap();
/// assert_eq!("one hundred myllion", one_hundred_myllion.as_str());
/// ```
pub fn power_of_ten(digits: &str) -> Result<String, ParseError> {
	// Sanity check. We want to convert our input string into a Bignum.
	// The num_bigint crate doesn't quite allow us to know the cause of
	// error, but from what we can tell, it's either an invalid digit or
	// an empty string. So we'll make this clear in our own error.
	let mut power = is_all_digits(digits)
		.then(|| digits)
		.ok_or(ParseError::InvalidDigit)
		.and_then(|d| BigUint::from_str(d).map_err(|_| ParseError::Empty))?;

	// Consider small cases
	let s = (&power % 2u32)
		.to_u32()
		.map(|m| match m { 0 => "one", 1 => "ten", _ => "" })
		.unwrap_or("");

	let mut output = String::from(s);

	power /= 2u32;
	let m = (&power % 2u32).to_u32();
	if m == Some(1) { output.push_str(" hundred"); }

	power /= 2u32;
	let m = (&power % 2u32).to_u32();
	if m == Some(2) { output.push_str(" myriad"); }

	// Break down the power one bit at a time, each time adding a new term.
	let mut zyl_num = 1;
	while !power.is_zero() {
		power /= 2u32;

		if zyl_num > 999 { 
			return Err(ParseError::InputTooLarge);
		}

		let m = (&power % 2u32).to_u32();
		if m == Some(1) {
			let prefix = latin_prefix(zyl_num)?;
			output.push(' ');
			output.push_str(prefix.as_str());
			output.push_str("yllion");
		}

		zyl_num += 1;		
	}

	Ok(output)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn small_numbers() {
		let forty_two_hundred = full_name("4200").unwrap();
		assert_eq!("forty two hundred", forty_two_hundred.as_str());
	}

	#[test]
	fn very_large_numbers() {
		// This test is taken verbatim using the example from
		// Knuth's essay, "Supernatural Numbers"
		let knuth_example = "\
			8065817517094387\
			8571660636856403\
			7669752895054408\
			83277824000000000000";
		let knuth_expected = "\
			eighty hundred sixty five quadryllion \
			eighty one hundred seventy five myriad \
			seventeen hundred nine myllion \
			forty three hundred eighty seven myriad \
			eighty five hundred seventy one byllion \
			sixty six hundred six myriad \
			thirty six hundred eighty five myllion \
			sixty four hundred three myriad \
			seventy six hundred sixty nine tryllion \
			seventy five hundred twenty eight myriad \
			ninety five hundred five myllion \
			forty four hundred eight myriad \
			eighty three hundred twenty seven byllion \
			seventy eight hundred twenty four myriad myllion";
		let example_result = full_name(knuth_example).unwrap();
		assert_eq!(knuth_expected, example_result.as_str());
	}

	#[test]
	fn semi_large_power() {
		let ten_to_the_forty_second = power_of_ten("42").unwrap();
		assert_eq!(
			"one hundred myllion tryllion",
			ten_to_the_forty_second.as_str()
		);
	}
}