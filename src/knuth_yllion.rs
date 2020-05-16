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

pub fn full_name(digits: &str) -> Result<String, &'static str> {
    Err("Not implemented")
}

pub fn power_of_ten(digits: &str) -> Result<String, &'static str> {
    Err("Not implemented")
}
