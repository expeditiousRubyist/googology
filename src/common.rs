/* This is an internal use module intended to provide functionality common
 * to both the -illion system used by Conway-Weschler and the -yllion system
 * used by Knuth.
 */

// Arrays used in the construction of latin prefixes
// The base prefixes are used for values 0 - 9, while the units, tens, and
// hundreds are used for values 10 - 999.
static LATIN_BASE_PREFIXES: [&'static str; 10] = [
	"n", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"
];

static LATIN_UNIT_PREFIXES: [&'static str; 10] = [
	"", "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo",
	"nove"
];

static LATIN_TENS_PREFIXES: [&'static str; 10] = [
	"", "deci", "viginti", "triginta", "quadraginta", "quinquaginta",
	"sexaginta", "septuaginta", "octoginta", "nonaginta"
];

static LATIN_HUNDREDS_PREFIXES: [&'static str; 10] = [
	"", "centi", "ducenti", "trecenti", "quadringenti", "quingenti",
	"sescenti", "septingenti", "octingenti", "nongenti"
];

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