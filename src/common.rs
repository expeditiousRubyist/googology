/* This is an internal use module intended to provide functionality common
 * to both the -illion system used by Conway-Weschler and the -yllion system
 * used by Knuth.
 */

// Substrings used to construct names for the numbers 1-100.
// These are used by the myriad_number function.
static NAMES_UPTO_TWENTY: [&str; 20] = [
	"", "one", "two", "three", "four", "five", "six", "seven", "eight",
	"nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
	"sixteen", "seventeen", "eighteen", "nineteen"
];

static TENS_NAMES: [&str; 10] = [
	"", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy",
	"eighty", "ninety"
];

// Arrays used in the construction of latin prefixes
// The base prefixes are used for values 0 - 9, while the units, tens, and
// hundreds are used for values 10 - 999.
static LATIN_BASE_PREFIXES: [&str; 10] = [
	"n", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"
];

static LATIN_UNIT_PREFIXES: [&str; 10] = [
	"", "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo",
	"nove"
];

static LATIN_TENS_PREFIXES: [&str; 10] = [
	"", "deci", "viginti", "triginta", "quadraginta", "quinquaginta",
	"sexaginta", "septuaginta", "octoginta", "nonaginta"
];

static LATIN_HUNDREDS_PREFIXES: [&str; 10] = [
	"", "centi", "ducenti", "trecenti", "quadringenti", "quingenti",
	"sescenti", "septingenti", "octingenti", "nongenti"
];

// Test an input string to see if it contains anything other than 0-9.
pub fn is_all_digits(s: &str) -> bool {
	s.chars().all(|c| c.is_digit(10))
}

// Casts a slice of a string of all digits into usize.
// This performs an unwrap twice, but since we should already verify that all
// characters in the string are numbers, this should never panic.
pub fn num_from_slice(digits: &str, index: usize, ndigits: usize) -> usize {
	digits.get(index..index+ndigits).unwrap().parse::<usize>().unwrap()
}

// Provides a prefix for some "-illion" or "-yllion" number.
// num should be some value between 0 and 999, or else None is returned
// for sanity's sake. Number names with complex prefixes such as millinillion
// will need to first break the num into powers of 1000 and invoke this function
// multiple times.
pub fn latin_prefix(num: usize) -> Option<String> {
	// Sanity check.
	if num >= 1000 { return None; }

	// We use the same latin prefix construction method used here
	// http://www.mrob.com/pub/math/largenum.html#conway-wechsler
	if num < 10 { 
		return Some(String::from(LATIN_BASE_PREFIXES[num]));
	}

	let hs = num / 100;      // Hundreds place
	let ts = num % 100 / 10; // Tens place
	let us = num % 10;       // Units place

	let mut prefix = String::from(LATIN_UNIT_PREFIXES[us]);

	// The position of each prefix is (units)(tens)(hundreds).
	// The unit place has special endings that sometimes give it an extra
	// letter depending on what should follow it. This is to distinguish
	// certain very similar prefixes apart (i.e. trecenti vs trescenti,
	// the latter of which being constructed from the tre unit prefix).
	if ts > 0 {
		// Special unit place endings
		match (us, ts) {
			(3, 2..=5) | (3, 8) => prefix.push('s'), // tres
			(6, 2..=5)          => prefix.push('s'), // ses
			(6, 8)              => prefix.push('x'), // sex
			(7, 1) | (7, 3..=7) => prefix.push('n'), // septen
			(7, 2) | (7, 8)     => prefix.push('m'), // septem
			(9, 1) | (9, 3..=7) => prefix.push('n'), // noven
			(9, 2) | (9, 8)     => prefix.push('m'), // novem
			_ => (),
		}

		prefix.push_str(LATIN_TENS_PREFIXES[ts]);
		prefix.push_str(LATIN_HUNDREDS_PREFIXES[hs]);
	}
	else {
		// Special unit place endings
		match (us, hs) {
			(3, 1) | (3, 3..=5) | (3, 8) => prefix.push('s'), // tres
			(6, 1) | (6, 8) => prefix.push('x'), // sex
			(6, 3..=5)      => prefix.push('s'), // ses
			(7, 1..=7)      => prefix.push('n'), // septen
			(7, 8)          => prefix.push('m'), // septem
			(9, 1..=7)      => prefix.push('n'), // noven
			(9, 8)          => prefix.push('m'), // novem
			_ => (),
		}

		prefix.push_str(LATIN_HUNDREDS_PREFIXES[hs]);
	}

	// Get rid fo the vowel at the end, which is replaced with either
	// an -illion or -yllion.
	prefix.pop();
	Some(prefix)
}

// Helper function for myriad number
// Generates a name for a number in the range [0,99].
// The name for the number is the empty string.
fn name_hundreds(tens: usize, units: usize) -> String {
	let mut output = String::from(TENS_NAMES[tens]);

	if output.is_empty() { 
		output.push_str(NAMES_UPTO_TWENTY[(10*tens)+units]);
	}
	else if units > 0 {
		output.push(' ');
		output.push_str(NAMES_UPTO_TWENTY[units]);
	}

	output
}

// This function converts a number in the range [1, 9999] to English words.
// Due to its use in the -yllion system, this function will not use the word
// "thousand", however, instead preferring "ten hundred" or something similar.
// Although this function is also used by Conway-Wechsler, which does use
// thousands, this will not be a problem as that function will only use this
// for three digit numbers.
pub fn myriad_number(num: usize) -> Option<String> {
	if num >= 10000 {
		return None;
	}

	let ms = num / 1000;       // Thousands (milia) place
	let hs = num % 1000 / 100; // Hundreds place
	let ts = num % 100 / 10;   // Tens place
	let us = num % 10;         // Units place

	let mut output = name_hundreds(ms, hs);
	let     append = name_hundreds(ts, us);

	if !output.is_empty() { output.push_str(" hundred "); }
	if !append.is_empty() { 
		output.push_str(append.as_str());
		output.push(' ');
	}

	output.pop(); // Remove the space at the end
	Some(output)
}
