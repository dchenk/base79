//! Textual representation of and utility functions for base-79 fractional numbers with arbitrary precision.
//!
//! It can only represent numbers between 0 and 1, exclusive. The leading `0.` is omitted.
//!
//! Heavily inspired by [this article](https://www.figma.com/blog/realtime-editing-of-ordered-sequences/).
//!
//! ## Why 79?
//!
//! - UTF-8 can encode ASCII with 1 byte per character.
//! - ASCII has 95 printable characters in total, but some of them will seem really odd if they were
//!   to surface to end-users. We could take just the alphanumeric characters (62), but that seems
//!   too limited. We take the middle 79 to exclude some of the characters on the ends, such as the
//!   space, which isn't very conspicuous when reading, and quote marks, which often need escaping.
//!
//! ## Example
//!
//! ```
//! use base79::Base79;
//! use std::str::FromStr;
//!
//! let n1 = Base79::mid();
//! assert_eq!(n1.to_string(), "R");
//! assert_eq!(n1.raw_digits(), vec![39]);
//! assert_eq!(39 + 1 + 39, 79); // How we got 39.
//!
//! let n2 = Base79::avg_with_zero(&n1);
//! assert_eq!(n2.to_string(), ">");
//! assert_eq!(n2.raw_digits(), vec![19]);
//! assert_eq!(19.5*2.0 + 1.0 + 19.5*2.0, 79.0); // How we got 19.
//!
//! let n3 = Base79::avg_with_one(&n1);
//! assert_eq!(n3.to_string(), "f");
//! assert_eq!(n3.raw_digits(), vec![59]);
//!
//! let n4 = Base79::avg(&n1, &n2);
//! assert_eq!(n4.to_string(), "H");
//! assert_eq!(n4.raw_digits(), vec![29]);
//!
//! let n5 = Base79::from_str("s?Q^Z").unwrap();
//! assert_eq!(n5.raw_digits(), vec![72, 20, 38, 51, 47]);
//! ```
//!
//! ## Why is `avg` imprecise?
//!
//! One of main considerations of this representation is storage efficiency of fractional index.
//! So it is better to have a little imprecise, shorter string, than perfectly precise, longer string.
//!
//! Of course, the result is deterministic, i.e., if the input is same, the output will always be same.

use crate::digits::Digits;

mod digits;

const MINIMUM: u8 = '+' as u8;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Base79(String);

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidChar,
    EmptyNotAllowed,
}

impl Base79 {
    /// Create a fractional number of base 79 in the middle of the 79-digit alphabet.
    /// The only way to create a Base79 instance without any arguments.
    pub fn mid() -> Self {
        Digits::mid().into()
    }

    pub fn avg(lhs: &Self, rhs: &Self) -> Self {
        Digits::avg(&lhs.into(), &rhs.into()).into()
    }

    pub fn avg_with_zero(n: &Self) -> Self {
        Digits::avg(&Digits::zero(), &n.into()).into()
    }

    pub fn avg_with_one(n: &Self) -> Self {
        Digits::avg(&Digits::one(), &n.into()).into()
    }

    pub fn raw_digits(&self) -> Vec<u8> {
        Digits::from(self).0
    }
}

impl ToString for Base79 {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::str::FromStr for Base79 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::EmptyNotAllowed)
        } else if s.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
            Err(ParseError::InvalidChar)
        } else {
            Ok(Base79(s.to_owned()))
        }
    }
}

impl From<Digits> for Base79 {
    fn from(digits: Digits) -> Self {
        Self(String::from_utf8(digits.0.iter().map(|x| x + MINIMUM).collect()).unwrap())
    }
}

impl From<&Base79> for Digits {
    fn from(base79: &Base79) -> Self {
        Self(base79.0.as_bytes().iter().map(|x| x - MINIMUM).collect())
    }
}

impl From<Base79> for String {
    fn from(base79: Base79) -> Self {
        base79.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Base79::from_str(""), Err(ParseError::EmptyNotAllowed));
        assert_eq!(Base79::from_str("한글"), Err(ParseError::InvalidChar));
        assert_eq!(Base79::from_str("R").unwrap(), Base79::mid());
    }
}
