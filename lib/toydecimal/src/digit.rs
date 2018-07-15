use std::{mem, slice};
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::ops::{Add, Sub, Mul, AddAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}
impl Digit {
    #[inline]
    pub fn new(digit: u8) -> Digit {
        assert!(digit <= 9, "Invalid digit: {}", digit);
        unsafe { Digit::new_unchecked(digit) }
    }
    #[inline]
    pub unsafe fn new_unchecked(digit: u8) -> Digit {
        mem::transmute::<u8, Digit>(digit)
    }
    #[inline]
    pub fn value(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn to_char(self) -> char {
        (b'0' + self.value()) as char
    }
    #[inline]
    pub fn from_char(c: char) -> Result<Digit, InvalidDigitErr> {
        if c >= '0' && c <= '9' {
            Ok(Digit::new((c as u8) - b'0'))
        } else {
            Err(InvalidDigitErr(c))
        }
    }
    #[inline]
    pub fn wrapping_add(self, rhs: Digit) -> Digit {
        Digit::new((self.value() + rhs.value()) % 10)
    }
    #[inline]
    pub fn wrapping_sub(self, rhs: Digit) -> Digit {
        Digit::new((self.value().wrapping_sub(rhs.value())) % 10)
    }
    #[inline]
    pub fn wrapping_mul(self, rhs: Digit) -> Digit {
        self.overflowing_mul(rhs).0
    }
    #[inline]
    pub fn overflowing_add(self, rhs: Digit) -> (Digit, bool) {
        let value = self.wrapping_add(rhs);
        (value, value < self)
    }
    #[inline]
    pub fn overflowing_sub(self, rhs: Digit) -> (Digit, bool) {
        let value = self.wrapping_sub(rhs);
        (value, value > self)
    }
    #[inline]
    pub fn overflowing_mul(self, rhs: Digit) -> (Digit, bool) {
        let value = self.value() * rhs.value();
        let wrapped = value % 10;
        (Digit::new(wrapped), wrapped != value)
    }
    #[inline]
    pub fn checked_add(self, rhs: Digit) -> Option<Digit> {
        let (digit, overflowed) = self.overflowing_add(rhs);
        if overflowed { None } else { Some(digit) }
    }
    #[inline]
    pub fn checked_sub(self, rhs: Digit) -> Option<Digit> {
        let (digit, overflowed) = self.overflowing_sub(rhs);
        if overflowed { None } else { Some(digit) }
    }
    #[inline]
    pub fn checked_mul(self, rhs: Digit) -> Option<Digit> {
        let (digit, overflowed) = self.overflowing_mul(rhs);
        if overflowed { None } else { Some(digit) }
    }
    #[inline]
    pub fn slice_as_bytes(target: &[Digit]) -> &[u8] {
        assert_eq!(mem::size_of::<Digit>(), mem::size_of::<u8>());
        unsafe { slice::from_raw_parts(target.as_ptr() as *const u8, target.len()) }
    }
}
impl Add<Digit> for Digit {
    type Output = Digit;

    #[inline]
    fn add(self, rhs: Digit) -> Digit {
        if cfg!(debug_assertions) {
            self.checked_add(rhs).unwrap()
        } else {
            self.wrapping_add(rhs)
        }
    }
}
impl AddAssign<Digit> for Digit {
    #[inline]
    fn add_assign(&mut self, rhs: Digit) {
        *self = *self + rhs;
    }
}
impl Sub<Digit> for Digit {
    type Output = Digit;

    #[inline]
    fn sub(self, rhs: Digit) -> Digit {
        if cfg!(debug_assertions) {
            self.checked_sub(rhs).unwrap()
        } else {
            self.wrapping_sub(rhs)
        }
    }
}
impl Mul<Digit> for Digit {
    type Output = Digit;

    fn mul(self, rhs: Digit) -> Digit {
        if cfg!(debug_assertions) {
            self.checked_mul(rhs).unwrap()
        } else {
            self.wrapping_mul(rhs)
        }
    }
}
impl PartialEq<u8> for Digit {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.value() == *other
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InvalidDigitErr(pub char);
impl Display for InvalidDigitErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid digit: {:?}", self.0)
    }
}
impl Error for InvalidDigitErr {}