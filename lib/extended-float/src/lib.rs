//! Extended precision floating point
#![feature(const_fn, proc_macro, proc_macro_non_items, ptr_offset_from)]
extern crate libc;
extern crate num_traits;

extern crate extended_float_sys as sys;
extern crate extended_float_macros;

use std::num::FpCategory;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, Neg, DivAssign, Rem, RemAssign};
use std::os::raw::{c_char};
use std::{hint, ptr, slice, mem};
use std::ffi::{CStr, CString, NulError};
use std::fmt::{self, Write, Debug, Display, Formatter};
use std::str::FromStr;
use std::cmp::Ordering;

use num_traits::{Num, Float, One, Zero, ToPrimitive};

use extended_float_macros::extended_float;

pub mod consts;

/// An extended precision floating point value.
///
/// This guarantees "at least" 80 bits of precision.
/// Currently this only works on x86 and maps directly to a 80-bit floating point value.
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtendedFloat([u8; 10]);

#[allow(dead_code)]
impl ExtendedFloat {
    #[inline]
    pub const fn to_bits(self) -> [u8; 10] {
        self.0
    }
    #[inline]
    pub const fn from_bits(bits: [u8; 10]) -> ExtendedFloat {
        ExtendedFloat(bits)
    }
    fn write<W: Write>(&self, width: Option<usize>, precision: Option<usize>, mut out: W) -> fmt::Result {
        self.print_with(width, precision, |data| {
            out.write_str(data.to_str().unwrap())
        })
    }
    fn print_with<R, F: FnOnce(&CStr) -> R>(&self, width: Option<usize>, precision: Option<usize>, func: F) -> R {
        let mut ptr: *mut c_char = ptr::null_mut();
        let size = unsafe { ::sys::extended_print(
            self.as_ptr(),
            width.map_or(0, |w| w as i32),
            precision.map_or(0, |w| w as i32),
            &mut ptr
        ) };
        assert!(size >= 0, "Out of memory");
        let data = unsafe {
            debug_assert_eq!(ptr.add(size as usize).read(), b'\0' as i8);
            CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(
                ptr as *const u8,
                (size as usize) + 1
            ))
        };
        let result = func(data);
        // Now free the memory with the `free` function
        unsafe {
            ::libc::free(ptr as *mut ::libc::c_void);
        }
        result
    }
    #[inline]
    fn as_ptr(&self) -> *const sys::ExtendedFloat {
        self as *const ExtendedFloat as *const sys::ExtendedFloat
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut sys::ExtendedFloat {
        self as *mut ExtendedFloat as *mut sys::ExtendedFloat
    }
    /// Decomposes given floating point value arg into integral and fractional parts
    #[inline]
    fn modf(mut self) -> (ExtendedFloat, ExtendedFloat) {
        unsafe {
            let mut int: ExtendedFloat = mem::uninitialized();
            sys::extended_modf(self.as_mut_ptr(), int.as_mut_ptr());
            (int, self)
        }
    }
}
impl ToPrimitive for ExtendedFloat {
    fn to_i64(&self) -> Option<i64> {
        if self.is_normal() {
            let value = i64::from(*self);
            // If it roundtrips then it's legit
            if ExtendedFloat::from(value) == *self {
                return Some(value)
            }
        }
        None
    }

    fn to_u64(&self) -> Option<u64> {
        if self.is_normal() {
            let value = u64::from(*self);
            // If it roundtrips then it's legit
            if ExtendedFloat::from(value) == *self {
                return Some(value)
            }
        }
        None
    }

    fn to_f32(&self) -> Option<f32> {
        if self.is_finite() {
            let min: ExtendedFloat = f32::min_value().into();
            let max: ExtendedFloat = f32::max_value().into();
            if *self >= min && *self <= max {
                Some(f32::from(*self))
            } else {
                None
            }
        } else {
            Some(f32::from(*self))
        }
    }

    fn to_f64(&self) -> Option<f64> {
        if self.is_finite() {
            let min: ExtendedFloat = f64::min_value().into();
            let max: ExtendedFloat = f64::max_value().into();
            if *self >= min && *self <= max {
                Some(f64::from(*self))
            } else {
                None
            }
        } else {
            Some(f64::from(*self))
        }
    }
}
impl One for ExtendedFloat {
    #[inline]
    fn one() -> Self {
        extended_float!(1)
    }
}

impl Zero for ExtendedFloat {

    #[inline]
    fn zero() -> Self {
        extended_float!(0)
    }

    fn is_zero(&self) -> bool {
        *self == extended_float!(0.0) || *self == extended_float!(-0.0)
    }
}
impl ::num_traits::NumCast for ExtendedFloat {
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        if let Some(floating) = n.to_f64() {
            Some(<ExtendedFloat as From<_>>::from(floating))
        } else if let Some(signed) = n.to_i64() {
            Some(<ExtendedFloat as From<_>>::from(signed))
        } else if let Some(unsigned) = n.to_u64() {
            Some(<ExtendedFloat as From<_>>::from(unsigned))
        } else {
            None
        }
    }
}
impl Num for ExtendedFloat {
    type FromStrRadixErr = ();

    #[inline]
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, ()> {
        unimplemented!()
    }
}
impl Float for ExtendedFloat {
    #[inline]
    fn nan() -> Self {
        consts::NAN
    }
    #[inline]
    fn infinity() -> Self {
        consts::INFINITY
    }

    #[inline]
    fn neg_infinity() -> Self {
        consts::NEG_INFINITY
    }
    #[inline]
    fn neg_zero() -> Self {
        extended_float!(-0.0)
    }
    fn min_value() -> Self {
        unimplemented!()
    }
    fn min_positive_value() -> Self {
        unimplemented!()
    }
    fn max_value() -> Self {
        unimplemented!()
    }

    #[inline]
    fn is_nan(self) -> bool {
        unsafe { sys::extended_isnan(self.as_ptr()) }
    }

    #[inline]
    fn is_infinite(self) -> bool {
        unsafe { sys::extended_isinf(self.as_ptr()) }
    }
    #[inline]
    fn is_finite(self) -> bool {
        unsafe { sys::extended_isfinite(self.as_ptr()) }
    }

    #[inline]
    fn is_normal(self) -> bool {
        unsafe { sys::extended_isnormal(self.as_ptr()) }
    }
    fn classify(self) -> FpCategory {
        if self.is_nan() {
            FpCategory::Nan
        } else if self.is_infinite() {
            FpCategory::Infinite
        } else if self.is_normal() {
            FpCategory::Normal
        } else if self == extended_float!(0.0) || self == extended_float!(-0.0) {
            FpCategory::Zero
        } else {
            FpCategory::Subnormal
        }
    }
    #[inline]
    fn floor(mut self) -> ExtendedFloat {
        unsafe { sys::extended_floor(self.as_mut_ptr()) }
        self
    }

    #[inline]
    fn ceil(mut self) -> ExtendedFloat {
        unsafe { sys::extended_ceil(self.as_mut_ptr()) }
        self
    }

    #[inline]
    fn round(mut self) -> ExtendedFloat {
        unsafe { sys::extended_round(self.as_mut_ptr()) }
        self
    }

    #[inline]
    fn trunc(mut self) -> Self {
        unsafe {
            sys::extended_trunc(self.as_mut_ptr());
            self
        }
    }

    #[inline]
    fn fract(self) -> Self {
        self.modf().1
    }

    #[inline]
    fn abs(mut self) -> ExtendedFloat {
        unsafe { sys::extended_abs(self.as_mut_ptr()) }
        self
    }

    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::nan()
        } else if self.is_sign_positive() {
            extended_float!(1)
        } else {
            extended_float!(0)
        }
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        unsafe { sys::extended_signbit(self.as_ptr()) == 0 }
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        !self.is_sign_positive()
    }

    #[inline]
    fn mul_add(mut self, a: Self, b: Self) -> Self {
        unsafe {
            sys::extended_mul_add(self.as_mut_ptr(), a.as_ptr(), b.as_ptr());
            self
        }
    }

    #[inline]
    fn recip(self) -> Self {
        extended_float!(1.0) / self
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powf(ExtendedFloat::from(n))
    }

    #[inline]
    fn powf(mut self, n: Self) -> Self {
        unsafe {
            sys::extended_pow(self.as_mut_ptr(), n.as_ptr());
            self
        }
    }

    #[inline]
    fn sqrt(mut self) -> ExtendedFloat {
        unsafe { sys::extended_sqrt(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn exp(mut self) -> Self {
        unsafe { sys::extended_exp(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn exp2(mut self) -> Self {
        unsafe { sys::extended_exp2(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn ln(mut self) -> Self {
        unsafe { sys::extended_ln(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        self.log2() / base.log2()
    }

    #[inline]
    fn log2(mut self) -> Self {
        unsafe { sys::extended_log2(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn log10(mut self) -> Self {
        unsafe { sys::extended_log10(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn max(mut self, other: ExtendedFloat) -> ExtendedFloat {
        unsafe { sys::extended_max(self.as_mut_ptr(), other.as_ptr()) };
        self
    }

    #[inline]
    fn min(mut self, other: ExtendedFloat) -> ExtendedFloat {
        unsafe { sys::extended_min(self.as_mut_ptr(), other.as_ptr()) };
        self
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        if self <= other {
            extended_float!(0.0)
        } else {
            self - other
        }
    }

    fn cbrt(mut self) -> Self {
        unsafe { sys::extended_cbrt(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn hypot(mut self, other: Self) -> Self {
        unsafe { sys::extended_hypot(self.as_mut_ptr(), other.as_ptr()) };
        self
    }

    #[inline]
    fn sin(mut self) -> Self {
        unsafe { sys::extended_sin(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn cos(mut self) -> Self {
        unsafe { sys::extended_cos(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn tan(mut self) -> Self {
        unsafe { sys::extended_tan(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn asin(mut self) -> Self {
        unsafe { sys::extended_asin(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn acos(mut self) -> Self {
        unsafe { sys::extended_acos(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn atan(mut self) -> Self {
        unsafe { sys::extended_atan(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn atan2(mut self, other: Self) -> Self {
        unsafe { sys::extended_atan2(self.as_mut_ptr(), other.as_ptr()) };
        self
    }

    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    #[inline]
    fn exp_m1(mut self) -> Self {
        unsafe { sys::extended_exp_m1(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn ln_1p(mut self) -> Self {
        unsafe { sys::extended_ln_1p(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn sinh(mut self) -> Self {
        unsafe { sys::extended_sinh(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn cosh(mut self) -> Self {
        unsafe { sys::extended_cosh(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn tanh(mut self) -> Self {
        unsafe { sys::extended_tanh(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn asinh(mut self) -> Self {
        unsafe { sys::extended_sinh(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn acosh(mut self) -> Self {
        unsafe { sys::extended_cosh(self.as_mut_ptr()) };
        self
    }

    #[inline]
    fn atanh(mut self) -> Self {
        unsafe { sys::extended_atanh(self.as_mut_ptr()) };
        self
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        unimplemented!()
    }
}
impl From<f64> for ExtendedFloat {
    #[inline]
    fn from(data: f64) -> Self {
        unsafe {
            let mut out: ExtendedFloat = mem::uninitialized();
            sys::extended_convert_from_f64(out.as_mut_ptr(), data);
            out
        }
    }
}

impl From<f32> for ExtendedFloat {
    #[inline]
    fn from(data: f32) -> Self {
        unsafe {
            let mut out: ExtendedFloat = mem::uninitialized();
            sys::extended_convert_from_f32(out.as_mut_ptr(), data);
            out
        }
    }
}

impl From<i64> for ExtendedFloat {
    #[inline]
    fn from(data: i64) -> Self {
        unsafe {
            let mut out: ExtendedFloat = mem::uninitialized();
            sys::extended_convert_from_i64(out.as_mut_ptr(), data);
            out
        }
    }
}

impl From<u64> for ExtendedFloat {
    #[inline]
    fn from(data: u64) -> Self {
        unsafe {
            let mut out: ExtendedFloat = mem::uninitialized();
            sys::extended_convert_from_u64(out.as_mut_ptr(), data);
            out
        }
    }
}

impl From<i32> for ExtendedFloat {
    #[inline]
    fn from(data: i32) -> Self {
        ExtendedFloat::from(data as i64)
    }
}

impl From<ExtendedFloat> for f64 {
    #[inline]
    fn from(first: ExtendedFloat) -> Self {
        unsafe { sys::extended_convert_into_f64(first.as_ptr()) }
    }
}

impl From<ExtendedFloat> for f32 {
    #[inline]
    fn from(first: ExtendedFloat) -> Self {
        unsafe { sys::extended_convert_into_f32(first.as_ptr()) }
    }
}

impl From<ExtendedFloat> for i64 {
    #[inline]
    fn from(first: ExtendedFloat) -> Self {
        unsafe { sys::extended_convert_into_i64(first.as_ptr()) }
    }
}

impl From<ExtendedFloat> for u64 {
    #[inline]
    fn from(first: ExtendedFloat) -> Self {
        unsafe { sys::extended_convert_into_u64(first.as_ptr()) }
    }
}
impl Add for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn add(mut self, rhs: ExtendedFloat) -> ExtendedFloat {
        self += rhs;
        self
    }
}
impl AddAssign for ExtendedFloat {
    #[inline]
    fn add_assign(&mut self, rhs: ExtendedFloat) {
        unsafe { sys::extended_add(self.as_mut_ptr(), rhs.as_ptr()) }
    }
}

impl Sub for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn sub(mut self, rhs: ExtendedFloat) -> ExtendedFloat {
        self -= rhs;
        self
    }
}
impl SubAssign for ExtendedFloat {
    #[inline]
    fn sub_assign(&mut self, rhs: ExtendedFloat) {
        unsafe { sys::extended_sub(self.as_mut_ptr(), rhs.as_ptr()) }
    }
}

impl Mul for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn mul(mut self, rhs: ExtendedFloat) -> ExtendedFloat {
        self *= rhs;
        self
    }
}
impl MulAssign for ExtendedFloat {
    #[inline]
    fn mul_assign(&mut self, rhs: ExtendedFloat) {
        unsafe { sys::extended_mul(self.as_mut_ptr(), rhs.as_ptr()) }
    }
}

impl Div for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn div(mut self, rhs: ExtendedFloat) -> ExtendedFloat {
        self /= rhs;
        self
    }
}
impl DivAssign for ExtendedFloat {
    #[inline]
    fn div_assign(&mut self, rhs: ExtendedFloat) {
        unsafe { sys::extended_div(self.as_mut_ptr(), rhs.as_ptr()) }
    }
}


impl Rem for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn rem(mut self, rhs: ExtendedFloat) -> ExtendedFloat {
        self %= rhs;
        self
    }
}
impl RemAssign for ExtendedFloat {
    #[inline]
    fn rem_assign(&mut self, rhs: ExtendedFloat) {
        unsafe { sys::extended_mod(self.as_mut_ptr(), rhs.as_ptr()) }
    }
}
impl PartialEq for ExtendedFloat {
    #[inline]
    fn eq(&self, other: &ExtendedFloat) -> bool {
        unsafe { sys::extended_eq(self.as_ptr(), other.as_ptr()) }
    }
}

impl PartialOrd for ExtendedFloat {
    #[inline]
    fn partial_cmp(&self, other: &ExtendedFloat) -> Option<Ordering> {
        unsafe {
            match sys::extended_cmp(self.as_ptr(), other.as_ptr()) {
                1 => Some(Ordering::Greater),
                0 => Some(Ordering::Equal),
                -1 => Some(Ordering::Less),
                -2 => None,
                code => {
                    if cfg!(debug_assertions) {
                        panic!("Unexpected code: {}", code)
                    }
                    hint::unreachable_unchecked()
                }
            }
        }
    }
}
impl Neg for ExtendedFloat {
    type Output = ExtendedFloat;

    #[inline]
    fn neg(mut self) -> ExtendedFloat {
        unsafe {
            sys::extended_neg(self.as_mut_ptr());
            self
        }
    }
}

impl Display for ExtendedFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.write(f.width(), f.precision(), f)
    }
}
impl Debug for ExtendedFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.write(f.width(), f.precision(), f)
    }
}
impl FromStr for ExtendedFloat {
    type Err = ExtendedFloatParseError;

    fn from_str(s: &str) -> Result<Self, ExtendedFloatParseError> {
        if s.is_empty() {
            Err(ExtendedFloatParseError::Empty)
        } else if s.chars().next().unwrap().is_whitespace() {
            Err(ExtendedFloatParseError::LeadingWhitespace(s.chars().next().unwrap()))
        } else {
            let data = CString::new(s.as_bytes())?;
            let mut end = ptr::null_mut();
            unsafe {
                let mut out: ExtendedFloat = mem::uninitialized();
                sys::extended_parse(out.as_mut_ptr(), data.as_ptr(), &mut end);
                let consumed_bytes = end.offset_from(data.as_ptr()) as usize;
                assert!(consumed_bytes <= s.len());
                if consumed_bytes != s.len() {
                    Err(ExtendedFloatParseError::TrailingChars(s.len() - consumed_bytes))
                } else {
                    Ok(out)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ExtendedFloatParseError {
    Empty,
    NullByte,
    LeadingWhitespace(char),
    TrailingChars(usize),
    InvalidFloat
}
impl From<NulError> for ExtendedFloatParseError {
    #[inline]
    fn from(_: NulError) -> Self {
        ExtendedFloatParseError::NullByte
    }
}
