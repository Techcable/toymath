#![feature(const_fn, exact_size_is_empty, const_vec_new, plugin)]
#![plugin(quickcheck_macros)]
#[cfg(test)]
extern crate quickcheck;

extern crate itertools;
extern crate num_traits;
extern crate num_bigint;


use std::{iter, slice};
use std::ops::{Mul, Sub, Add};
use std::cmp::Ordering;

use self::num_traits::Zero;

mod digit;
mod test;
pub mod int;

use digit::Digit;

#[derive(Clone)]
pub struct Decimal {
    /// The sign of the decimal (whether it's negative),
    /// which must be ignored when the decimal is zero.
    sign: bool,
    /// The leading digit before the decimal point,
    /// which there can only be one of too statically verify
    /// we're in scientific notation.
    ///
    /// A zero as the leading zero represents a zero decimal,
    /// regardless of the sign or magnitude.
    /// Although there are multiple possible binary representations,
    /// they should all be equivalent to the user.
    leading: Digit,
    /// The normalized digits after the decimal point,
    /// without any trailing zeros.
    ///
    /// This allows us to have a single normalized representation
    /// for all numerically equivalent decimals,
    /// just like in IEEE floating point arithmetic.
    /// The prohibition of trailing zeros is **essential** for a normalized representation,
    /// as it allows fast comparisons without stripping the trailing zeros.
    magnitude: Vec<Digit>,
    /// The exponent is the power of ten we multiply the magnitude by to get the real value.
    ///
    /// This essentially represents the decimal using scientific notation,
    /// although it's completely ignored when the leading digit is `zero`.
    exponent: i32
}
impl Decimal {
    #[inline]
    pub const fn from_digit(digit: Digit) -> Decimal {
        Decimal {
            sign: false,
            leading: digit,
            magnitude: Vec::new(),
            exponent: 0
        }
    }
    #[inline]
    pub const fn zero() -> Decimal {
        Decimal::from_digit(Digit::Zero)
    }
    fn from_raw_decimal(
        sign: bool, mut exponent: i32,
        digits: &[Digit],
        mut decimal_point: usize
    ) -> Decimal {
        if digits.len() == 0 {
            return Decimal::zero()
        }
        let leading = digit[0];
        debug_assert_ne!(leading, Digit::Zero, "Leading zero: {:?}", digits);
        /*
         * The number of digits we want before the decimal point in order to be normalized.
         * We always need to reserve one digit afterwards to serve as the leading digit.
         */
        let expected_digits_before = digits.len() - 1;
        let delta = match decimal_point.cmp(&expected_digits_before) {
            Ordering::Greater => {
                /*
                 * The decimal point comes after all the digits,
                 * meaning we need to shift it to the right in order to normalize.
                 * Shifting the decimal to the right means we need to decrease the exponent.
                 */
                i32::try_from(decimal_point - expected_digits_before)
                    .and_then(i32::checked_neg).ok()
            },
            Ordering::Equal => Some(0),
            Ordering::Less => {
                /*
                 * The decimal point comes after all the digits,
                 * meaning we need to shift it to the left in order to normalize.
                 * Shifting the decimal to the left means we need to increase the exponent.
                 */
                i32::try_from(expected_digits_before - decimal_point).ok()
            }
        };
        let delta = delta.expect("Decimal overflow");
        exponent = exponent.checked_add(delta).expect("Exponent overflow");
        Decimal { leading, sign, exponent, magnitude: Vec::from(&digits[1..]), }
    }
    /// Returns `abs(self) + abs(other)`, essentially ignoring signs
    fn add_unsigned(&self, other: &Decimal) -> Decimal {
        let mut target_exponent = self.exponent.max(rhs.exponent);
        let mut left_digits = vec![Digit::Zero; target_exponent - self.exponent];
        left_digits.extend(self.magnitude.iter());
        left_digits.push(self.leading);
        let mut right_digits = vec![Digit::Zero; target_exponent - other.exponent];
        right_digits.extend(other.magnitude.iter());
        let mut right_decimal_point = right_digits.len();
        right_digits.push(other.leading);
        loop {
            let digits_before_left_decimal = left_decimal_point;
        }
    }
}
impl Add for Decimal {

}
impl Sub for Decimal {
    type Output = Decimal;

    #[inline]
    fn sub(self, mut rhs: Decimal) -> Decimal {
        rhs.sign = !rhs.sign;
        self + rhs
    }
}
impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        if self.is_zero() || other.is_zero() {
            return self
        }
        let sign = self.sign ^ other.sign;
        let mut exponent = self.exponent.checked_add(other.exponent)
            .expect("Exponent overflow");
        /*
         * When multiplying decimals you multiply them as if they were integers then
         * insert the decimal point by adding the decimal places of each of the inputs.
         * For example `1.2 * 0.4` as integers is `12 * 4`, which gives the integer result `48`.
         * Since there was one decimal in each input, we have two decimals in the result giving the decimal `.48`.
         * We're basically in scientific notation here, so a `.48` simply won't do.
         * We need to ensure that there is a single leading digit in the result,
         * by shifting the decimal point (changing the exponent)
         * before we finally convert back into a Decimal.
         */
        let mut raw_result = int::math::mul(RawDigits(&self), RawDigits(&other));
        let mut decimal_point = self.magnitude.len().checked_add(other.magnitude.len())
            .expect("Magnitude overflow");
        Decimal::from_raw_decimal(
            sign, exponent,
            raw_result.digits(), decimal_point
        )
    }
}
#[derive(Copy, Clone, Debug)]
struct RawDigits<'a>(&'a Decimal);
impl<'a> int::math::DecimalArith for RawDigits<'a> {
    type Iter = IterDigits<'a>;
    type Normalized = IterDigits<'a>;

    #[inline]
    fn len(self) -> usize {
        self.0.magnitude.len() + 1
    }

    #[inline]
    fn get(self, index: usize) -> Digit {
        if index == 0 {
            self.0.leading
        } else {
            self.0.magnitude[index - 1]
        }
    }
    #[inline]
    fn iter(self) -> Self::Iter {
        self.normalized_digits()
    }
    fn normalized_digits(&self) -> Self::Normalized {
        if self.is_zero() {
            IterDigits {
                digit: None,
                leading: [].iter(),
            }
        } else {
            debug_assert!(!self.0.leading.is_zero());
            IterDigits {
                digit: Some(self.0.leading),
                leading: self.0.magnitude.iter()
            }
        }
    }
}
#[derive(Clone, Debug)]
struct IterDigits<'a> {
    digit: Option<Digit>,
    leading: slice::Iter<'a, Digit>,
}
impl<'a> Iterator for IterDigits<'a> {
    type Item = Digit;

    #[inline]
    fn next(&mut self) -> Option<Digit> {
        if let Some(back) = self.leading.next_back() {
            Some(back)
        } else {
            self.digit.take()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.leading.len() + (self.digit.is_some() as usize);
        (len, Some(len))
    }
}
impl<'a> ExactSizeIterator for IterDigits<'a> {}
impl<'a> DoubleEndedIterator for IterDigits<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Digit> {
        if let Some(last) = self.digit.take() {
            Some(last)
        } else {
            self.leading.next()
        }
    }
}
impl Zero for Decimal {
    #[inline]
    fn zero() -> Self {
        Decimal::zero()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        let zero = self.leading == 0;
        debug_assert!(!zero || self.magnitude.is_empty());
        zero
    }
}
