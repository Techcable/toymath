//! Grade school decimal multiplication
use std::{slice, iter, option};
use std::cmp::Ordering;
use std::fmt::Debug;

use digit::Digit;
use int::DecimalInt;

/// A type that supports decimal arithmetic.
pub trait DecimalArith: Debug + Copy {
    type Iter: Iterator<Item=Digit>;
    type Normalized: Iterator<Item=Digit> + iter::ExactSizeIterator;
    fn len(self) -> usize;
    fn get(self, index: usize) -> Digit;
    fn iter(self) -> Self::Iter;
    fn normalized_digits(&self) -> Self::Normalized;
    #[inline]
    fn is_zero(self) -> bool {
        self.normalized_digits().is_empty()
    }
    #[inline]
    fn to_basic(self) -> DecimalInt {
        DecimalInt::from(self.iter().collect::<Vec<Digit>>())
    }
}
impl<'a> DecimalArith for &'a DecimalInt {
    type Iter = iter::Cloned<slice::Iter<'a, Digit>>;
    type Normalized = iter::Cloned<slice::Iter<'a, Digit>>;

    #[inline]
    fn len(self) -> usize {
        (*self).len()
    }

    #[inline]
    fn get(self, index: usize) -> Digit {
        (*self).get(index)
    }

    #[inline]
    fn iter(self) -> Self::Iter {
        (*self).digits().iter().cloned()
    }

    #[inline]
    fn normalized_digits(&self) -> Self::Normalized {
        (**self).normalized_digits().iter().cloned()
    }
    #[inline]
    fn to_basic(self) -> DecimalInt {
        (*self).clone()
    }
}
impl DecimalArith for Digit {
    type Iter = iter::Once<Digit>;
    type Normalized = option::IntoIter<Digit>;

    #[inline]
    fn len(self) -> usize {
        1
    }

    #[inline]
    fn get(self, index: usize) -> Digit {
        assert_eq!(index, 0, "Index out of bounds");
        self
    }

    #[inline]
    fn iter(self) -> Self::Iter {
        iter::once(self)
    }

    #[inline]
    fn normalized_digits(&self) -> Self::Normalized {
        if *self == Digit::Zero {
            None.into_iter()
        } else {
            Some(*self).into_iter()
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct BigEndianNormalizedSlice<'a>(pub &'a [Digit]);
impl<'a> DecimalArith for BigEndianNormalizedSlice<'a> {
    type Iter = iter::Rev<iter::Cloned<slice::Iter<'a, Digit>>>;
    type Normalized = iter::Rev<iter::Cloned<slice::Iter<'a, Digit>>>;

    #[inline]
    fn len(self) -> usize {
        self.0.len()
    }

    #[inline]
    fn get(self, index: usize) -> Digit {
        if index < self.0.len() {
            self.0[self.0.len() - 1 - index]
        } else {
            panic!("Invalid index: {}", index)
        }
    }

    #[inline]
    fn iter(self) -> Self::Iter {
        self.0.iter().cloned().rev()
    }

    #[inline]
    fn normalized_digits(&self) -> Self::Normalized {
        // By contract we are already normalized
        debug_assert!(self.0.last().map_or(true, |digit| *digit == Digit::Zero));
        self.iter()
    }
}

pub fn add<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> DecimalInt {
    let mut result = a.to_basic();
    while result.len() < b.len() {
        result.push(Digit::Zero);
    }
    add_inplace(&mut result, b);
    result
}
fn add_inplace<T: DecimalArith>(a: &mut DecimalInt, b: T) {
    assert!(b.len() <= a.len());
    let mut carry = false;
    for (first, second) in a.digits_mut().iter_mut()
        .zip(b.iter()) {
        let mut new_value = first.value() + second.value() + (if carry { 1 } else { 0 });
        if new_value >= 10 {
            new_value -= 10;
            carry = true;
        } else {
            carry = false;
        };
        *first = Digit::new(new_value);
    }
    if carry {
        for digit in a.digits_mut()[b.len()..].iter_mut() {
            let mut new_value = digit.value() + (if carry { 1 } else { 0 });
            if new_value >= 10 {
                new_value -= 10;
                carry = true;
            } else {
                carry = false;
            }
            *digit = Digit::new(new_value);
            if !carry { break }
        }
    }
    if carry {
        a.push(Digit::One);
    }
}

pub fn cmp<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> Ordering {
    let len = match a.len().cmp(&b.len()) {
        Ordering::Less => {
            for index in a.len()..b.len() {
                if b.get(index) != Digit::Zero {
                    return Ordering::Less
                }
            }
            a.len()
        },
        Ordering::Equal => a.len(),
        Ordering::Greater => {
            for index in b.len()..a.len() {
                if a.get(index) != Digit::Zero {
                    return Ordering::Greater
                }
            }
            b.len()
        }
    };
    debug_assert_eq!(len, a.len().min(b.len()));
    for index in (0..len).rev() {
        let ord = a.get(index).cmp(&b.get(index));
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

pub fn sub<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> DecimalInt {
    let (value, overflowed) = overflowing_sub(a, b);
    assert!(!overflowed, "Overflowed {:?} - {:?}", a, b);
    value
}


pub fn sub_inplace<T: DecimalArith>(a: &mut DecimalInt, b: T) {
    let overflowed = overflowing_sub_inplace(a, b);
    assert!(!overflowed, "Overflowed {:?} - {:?}", a, b);
}

pub fn overflowing_sub<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> (DecimalInt, bool) {
    let mut result = a.to_basic();
    while result.len() < b.len() {
        result.push(Digit::Zero);
    }
    let overflowed = overflowing_sub_inplace(&mut result, b);
    (result, overflowed)
}
pub fn overflowing_sub_inplace<T: DecimalArith>(a: &mut DecimalInt, b: T) -> bool {
    assert!(b.len() <= a.len());
    let mut borrow = false;
    for (first, second) in a.digits_mut().iter_mut().zip(b.iter()) {
        let needs_borrow = first.value() < (borrow as u8) + second.value();
        let mut new_value = (if needs_borrow { 10 } else { 0 })
            + first.value() - ((borrow as u8) + second.value());
        debug_assert!(new_value < 10);
        *first = Digit::new(new_value);
        borrow = needs_borrow;
    }
    if borrow {
        for digit in a.digits_mut()[b.len()..].iter_mut() {
            if *digit != Digit::Zero {
                *digit = Digit::new(digit.value() - 1);
                borrow = false;
                break
            }
        }
    }
    borrow
}

pub fn mul<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> DecimalInt {
    // NOTE: Algorithm assumes 'little endian' so the least significant digits come first.
    if a.len() < b.len() {
        return mul(b, a)
    }
    let mut product = vec![Digit::Zero; a.len() + b.len()];
    debug_assert!(b.len() <= a.len());
    for (b_i, b_digit) in b.iter().enumerate() {
        let mut carry = Digit::Zero;
        for (a_i, a_digit) in a.iter().enumerate() {
            let old_digit = product[a_i + b_i];
            let value = old_digit.value() + carry.value() + (a_digit.value() * b_digit.value());
            carry = Digit::new(value / 10);
            product[a_i + b_i] = Digit::new(value % 10);
        }
        product[b_i + a.len()] += carry;
    }
    DecimalInt::from(product)
}


pub fn div<A: DecimalArith, B: DecimalArith>(a: A, b: B) -> DecimalInt {
    // Our division 'algorithim' is to convert into base two and then divide
    assert!(!b.is_zero());
    (a.to_basic().as_bigint() / b.to_basic().as_bigint()).into()
}

#[cfg(test)]
mod test {
    use super::*;
    #[quickcheck]
    fn check_add(a: u64, b: u64) -> bool {
        add(&DecimalInt::from(a), &DecimalInt::from(b)) == DecimalInt::from(a + b)
    }
    #[test]
    fn basic_sub() {
        assert_eq!(
            sub(&DecimalInt::from(1), &DecimalInt::from(1)),
            DecimalInt::zero()
        )
    }
    #[quickcheck]
    fn check_sub(a: u64, b: u64) -> bool {
        a < b || sub(&DecimalInt::from(a), &DecimalInt::from(b)) == DecimalInt::from(a - b)
    }
    #[quickcheck]
    fn check_mul(a: u64, b: u64) {
        if let Some(expected) = a.checked_mul(b) {
            assert_eq!(
                mul(
                    &DecimalInt::from(a),
                    &DecimalInt::from(b)
                ),
                DecimalInt::from(expected),
                "Failed to multiply {} by {}",
                a, b
            );
        }
    }
    #[quickcheck]
    fn check_div(a: u64, b: u64) {
        if b != 0 {
            assert_eq!(
                div(
                    &DecimalInt::from(a),
                    &DecimalInt::from(b)
                ),
                DecimalInt::from(a / b),
                "Unable to divide {} by {}",
                a, b
            );
        }
    }
}