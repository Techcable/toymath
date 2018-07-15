use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

use super::digit::{Digit, InvalidDigitErr};
use std::fmt::{self, Write, Display, Debug, Formatter};

pub mod math;

/// This is 'little endian' so the least significant digits come first.
#[derive(Clone)]
pub struct DecimalInt(Vec<Digit>);
impl DecimalInt {
    #[inline]
    pub const fn zero() -> DecimalInt {
        DecimalInt(Vec::new())
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> DecimalInt {
        DecimalInt(Vec::with_capacity(capacity))
    }
    #[inline]
    pub fn digits(&self) -> &[Digit] {
        &self.0
    }
    #[inline]
    pub fn digits_mut(&mut self) -> &mut [Digit] {
        &mut self.0
    }
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|digit| Digit::Zero == *digit)
    }
    pub fn normalize(&mut self) {
        while !self.is_normalized() {
            self.0.pop();
        }
    }
    pub fn normalized_digits(&self) -> &[Digit] {
        let mut digits = self.digits();
        if !self.is_normalized() {
            while let Some((last, trimmed)) = digits.split_last() {
                if *last == Digit::Zero {
                    digits = trimmed;
                } else {
                    break
                }
            }
        }
        digits
    }
    #[inline]
    pub fn push(&mut self, value: Digit) {
        self.0.push(value)
    }
    #[inline]
    pub fn pop(&mut self) -> Option<Digit> {
        self.0.pop()
    }
    #[inline]
    pub fn is_normalized(&self) -> bool {
        self.0.last().map_or(true, |last| *last != Digit::Zero)
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        Digit::slice_as_bytes(&self.0)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=Digit> + 'a {
        self.0.iter().cloned()
    }
    #[inline]
    pub fn get(&self, index: usize) -> Digit {
        self.0[index]
    }
    #[inline]
    pub fn last(&self) -> Option<Digit> {
        self.0.last().cloned()
    }
    #[must_use]
    pub fn shifted_decimal_left(&self, amount: usize) -> DecimalInt {
        let mut result: Vec<Digit> = Vec::with_capacity(self.0.len() + amount);
        unsafe {
            result.as_mut_ptr().write_bytes(0, amount);
            result.as_mut_ptr().add(amount)
                .copy_from_nonoverlapping(self.0.as_ptr(), self.0.len());
            result.set_len(self.0.len() + amount);
        }
        DecimalInt(result)
    }
    /// Shift all the decimal digits to the left by `amount`,
    /// padding them with zeros to fill the newly created space.
    ///
    /// This effectively multiplies by `10**amount`
    /// and is actually much faster then regular multiplication or addition.
    pub fn shift_decimal_left(&mut self, amount: usize) {
        self.0.reserve(amount);
        unsafe {
            let len = self.0.len();
            let target_ptr = self.0.as_mut_ptr().add(amount);
            self.0.as_ptr().copy_to(target_ptr, len);
            assert_eq!(Digit::Zero as u8, 0u8);
            self.0.as_mut_ptr().write_bytes(0, amount);
            self.0.set_len(len + amount);
        }
    }
    pub fn as_bigint(&self) -> BigUint {
        let mut result = BigUint::new(Vec::with_capacity(self.len() * 4));
        for &digit in self.0.iter().rev() {
            result *= 10u8;
            result += digit.value();
        }
        result
    }
}
impl From<Vec<Digit>> for DecimalInt {
    #[inline]
    fn from(digits: Vec<Digit>) -> Self {
        DecimalInt(digits)
    }
}

impl From<Digit> for DecimalInt {
    #[inline]
    fn from(digit: Digit) -> Self {
        DecimalInt(vec![digit])
    }
}
impl From<DecimalInt> for Vec<Digit> {
    #[inline]
    fn from(decimal: DecimalInt) -> Self {
        decimal.0
    }
}
impl Index<usize> for DecimalInt {
    type Output = Digit;

    #[inline]
    fn index(&self, index: usize) -> &Digit {
        &self.0[index]
    }
}
impl IndexMut<usize> for DecimalInt {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Digit {
        &mut self.0[index]
    }
}
impl From<u64> for DecimalInt {
    fn from(mut value: u64) -> Self {
        let mut result = DecimalInt::zero();
        while value != 0 {
            let digit = Digit::new((value % 10) as u8);
            value /= 10;
            result.push(digit)
        }
        debug_assert!(result.is_normalized());
        result
    }
}
impl From<BigUint> for DecimalInt {
    fn from(mut value: BigUint) -> Self {
        let mut result = DecimalInt::zero();
        while !value.is_zero() {
            let digit = Digit::new((&value % 10u8).to_u8().unwrap());
            value /= 10u8;
            result.push(digit)
        }
        debug_assert!(result.is_normalized());
        result
    }
}

impl FromStr for DecimalInt {
    type Err = DecimalIntParseError;

    fn from_str(s: &str) -> Result<Self, DecimalIntParseError> {
        if s.is_empty() {
            return Err(DecimalIntParseError::EmptyStr)
        }
        let mut result = DecimalInt::with_capacity(s.len());
        for c in s.chars() {
            result.push(Digit::from_char(c)?);
        }
        assert!(!s.is_empty());
        result.normalize();
        Ok(result)
    }
}

pub enum DecimalIntParseError {
    EmptyStr,
    InvalidDigit(InvalidDigitErr)
}
impl From<InvalidDigitErr> for DecimalIntParseError {
    #[inline]
    fn from(cause: InvalidDigitErr) -> Self {
        DecimalIntParseError::InvalidDigit(cause)
    }
}
impl Display for DecimalInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0.is_empty() {
            f.write_char('0')?;
        } else {
            for &digit in self.0.iter().rev() {
                f.write_char(digit.to_char())?;
            }
        }
        Ok(())
    }
}
impl Debug for DecimalInt {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Ord for DecimalInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        math::cmp(self, other)
    }
}
impl PartialOrd for DecimalInt {
    #[inline]
    fn partial_cmp(&self, other: &DecimalInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for DecimalInt {
    #[inline]
    fn eq(&self, other: &DecimalInt) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for DecimalInt {}
