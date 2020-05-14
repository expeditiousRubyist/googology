//! Methods for the Conway-Wechsler large number naming system.
//! A proper description of how this system works can be found on
//! [this website](http://www.mrob.com/pub/math/largenum.html#conway-wechsler).
//! 
//! We add to this system just slightly by allowing the user to choose between
//! the short scale (thousand, million, billion, trillion...), which is the
//! standard in modern English, and the long scale (thousand, million, milliard,
//! billion...), which is more common in European languages, and which used to
//! be the standard for British English.

// Substrings used to construct names for the numbers 1-100.
static NAMES_UPTO_TWENTY: [&'static str; 20] = [
	"", "one", "two", "three", "four", "five", "six", "seven", "eight",
	"nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
	"sixteen", "seventeen", "eighteen", "nineteen"
];

static TENS_NAMES: [&'static str; 10] = [
	"", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy",
	"eighty", "ninety"
];

// Substrings used to construct names of larger numbers
// Since they all end in -illion (or -illiard in half of the names in the
// long count system), we call them "zillion" numbers.
static ZILLION_UNIT_PREFIXES: [&'static str; 10] = [
	"", "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo",
	"nove"
];

static ZILLION_TENS_PREFIXES: [&'static str; 10] = [
	"", "deci", "viginti", "triginta", "quadraginta", "quinquaginta",
	"sexaginta", "septuaginta", "octoginta", "nonaginta"
];

static ZILLION_HUNDREDS_PREFIXES: [&'static str; 10] = [
	"", "centi", "ducenti", "trecenti", "quadringenti", "quingenti",
	"sescenti", "septingenti", "octingenti", "nongenti"
];

static ZILLIONS_UNDER_TEN: [&'static str; 10] = [
	"nilli", "milli", "billi", "trilli", "quadrilli", "quintilli",
	"sextilli", "septilli", "octilli", "nonilli"
];

// Produces a name for some number in the range [0, 999].
// The name for zero is the empty string.
// Values above 999 will panic
fn three_digit_name(num: usize) -> String {
	assert!(num < 1000, "Input to three_digit_name is more than 3 digits!");

	let hs = num / 100;      // Hundreds place
	let ts = num % 100 / 10; // Tens place
	let us = num % 10;       // Units place

	// Hundred name (if applicable)
	let mut name = String::from(NAMES_UPTO_TWENTY[hs]);
	if !name.is_empty() { name.push_str(" hundred"); }

	// Rest of name
	if ts > 1 {
		if !name.is_empty() { name.push_str(" "); }
		name.push_str(TENS_NAMES[ts]);
		if us > 0 { 
			name.push_str(" ");
			name.push_str(NAMES_UPTO_TWENTY[us]);
		}
	}
	else {
		let aux = ts*10 + us;
		if aux > 0 {
			if !name.is_empty() { name.push_str(" "); }
			name.push_str(NAMES_UPTO_TWENTY[aux]);
		}
	}

	name
}

// Create a name for a single 3 digit zillion number, ending in -illi.
// Value for zero is "nilli", for use in chained zillion numbers.
// Values above 999 will panic.
fn zillion_prefix(num: usize) -> String {
	assert!(num < 1000, "Input to zillion_prefix is more than 3 digits!");

	if num < 10 { return String::from(ZILLIONS_UNDER_TEN[num]); }

	let hs = num / 100;      // Hundreds place
	let ts = num % 100 / 10; // Tens place
	let us = num % 10;       // Units place

	let mut name = String::from(ZILLION_UNIT_PREFIXES[us]);
	if ts > 0 {
		// Special unit place endings
		match (us, ts) {
			(3, 2..=5) | (3, 8) => name.push('s'), // tres
			(6, 2..=5)          => name.push('s'), // ses
			(6, 8)              => name.push('x'), // sex
			(7, 1) | (7, 3..=7) => name.push('n'), // septen
			(7, 2) | (7, 8)     => name.push('m'), // septem
			(9, 1) | (9, 3..=7) => name.push('n'), // noven
			(9, 2) | (9, 8)     => name.push('m'), // novem
			_ => (),
		}

		name.push_str(ZILLION_TENS_PREFIXES[ts]);
		name.push_str(ZILLION_HUNDREDS_PREFIXES[hs]);
	}
	else {
		// Special unit place endings
		match (us, hs) {
			(3, 1) | (3, 3..=5) | (3, 8) => name.push('s'), // tres
			(6, 1) | (6, 8) => name.push('x'), // sex
			(6, 3..=5)      => name.push('s'), // ses
			(7, 1..=7)      => name.push('n'), // septen
			(7, 8)          => name.push('m'), // septem
			(9, 1..=7)      => name.push('n'), // noven
			(9, 8)          => name.push('m'), // novem
			_ => (),
		}

		name.push_str(ZILLION_HUNDREDS_PREFIXES[hs]);
	}

	name.pop();
	name.push_str("illi");
	name
}

// Create a name for an arbitrary power of 1000.
// Value for zero is the empty string.
// Value for one is "thousand".
// Value for anything greater will involve repeated application of the
// zillion_prefix function, to create a number ending in "illion".
fn zillion_number(num: usize) -> String {
	if num == 0 { return String::from(""); }
	if num == 1 { return String::from("thousand"); }
	
	let mut name  = String::from("");
	let mut power = num - 1;

	// Prefixes technically added in reverse order here.
	// e.g. in millinillion, first add "nilli", then "milli", then "on".
	while power > 0 {
		let prefix = zillion_prefix(power % 1000);
		name.insert_str(0, prefix.as_str());
		power /= 1000;
	}

	name.push_str("on");
	name
}

// Test an input string to see if it contains anything other than 0-9.
fn is_all_digits(s: &str) -> bool {
	s.chars().all(|c| c.is_digit(10))
}

/// Gives a full length name for a number represented by an arbitrary sequence
/// of digits (example: "nineteen thousand fourty two" for "19042").
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
pub fn full_name(digits: &str, short: bool) -> Result<String, &'static str> {
	// Sanity check
	if !is_all_digits(digits) {
		return Err("digits should only contain the values 0-9")
	}

	// Skip leading zeroes. If all characters are 0, return "zero"
	let tmp = digits.find(|c| c != '0');
	let mut output = match tmp {
		Some(_) => String::from(""),
		None => String::from("zero"),
	};

	if !output.is_empty() { return Ok(output); }

	// Determine how many digits to process, and how many digits are in the
	// first zillion (e.g. 2 in the case of 12 tredecillion).
	let mut i = tmp.unwrap();
	let mut remaining = digits.len() - i;
	let first = remaining % 3;

	if first > 0 {
		let num = digits.get(i..i+first)
		                .unwrap()
		                .parse::<usize>()
		                .unwrap();
		let leading = three_digit_name(num);
		let zillion = zillion_number(remaining / 3);

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
		let num = digits.get(i..i+3)
		                .unwrap()
		                .parse::<usize>()
		                .unwrap();
		let leading = three_digit_name(num);
		let zillion = zillion_number(remaining / 3 - 1);

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
/// let milliard = power_of_ten("9", false).unwrap();
/// let billion = power_of_ten("9", true).unwrap();
/// assert_eq!("one milliard", milliard.as_str());
/// assert_eq!("one billion", billion.as_str());
/// ```
pub fn power_of_ten(digits: &str, short: bool) -> Result<String, &'static str> {
	Err("Not implemented")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn very_small_numbers() {
		let zero_ss = full_name("0", true).unwrap();
		let zero_ls = full_name("0", false).unwrap();
		assert_eq!("zero", zero_ss.as_str());
		assert_eq!("zero", zero_ls.as_str());		
	}

	#[test]
	fn small_numbers() {
		let twelve_ss = full_name("12", true).unwrap();
		let twelve_ls = full_name("12", false).unwrap();
		assert_eq!("twelve", twelve_ss.as_str());
		assert_eq!("twelve", twelve_ls.as_str());
	}

	#[test]
	fn large_numbers() {
		let billion = full_name("1000000000", true).unwrap();
		let milliard = full_name("1000000000", false).unwrap();
		assert_eq!("one milliard", milliard.as_str());
		assert_eq!("one billion", billion.as_str());
	}
}