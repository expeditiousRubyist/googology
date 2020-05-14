//! Methods for the Conway-Wechsler large number naming system.
//! A proper description of how this system works can be found on
//! [this website](http://www.mrob.com/pub/math/largenum.html#conway-wechsler).
//! 
//! We add to this system just slightly by allowing the user to choose between
//! the short scale (thousand, million, billion, trillion...), which is the
//! standard in modern English, and the long scale (thousand, million, milliard,
//! billion...), which is more common in European languages, and which used to
//! be the standard for British English.

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
	Err("Not implemented")
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