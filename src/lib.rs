//! # Googology
//! Googlogy is the [study and nomenclature of large numbers.](https://googology.wikia.org/wiki/Googology)
//! This crate is focused on the latter component of that description,
//! and provides utilities for converting numbers of arbitrarily large size
//! to some English-language description.
//! 
//! Currently, this crate only supports the Conway-Wechsler system for
//! naming numbers, however other systems may be added in the future.

pub mod conway_wechsler;
