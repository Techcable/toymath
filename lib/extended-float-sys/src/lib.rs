#![feature(const_fn)]
extern crate libc;

use libc::{c_int, c_uint, c_char};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct ExtendedFloat(pub [u8; 10]);
impl ExtendedFloat {
    /// Stub method for macros
    #[inline]
    #[doc(hidden)]
    pub const fn from_bits(bits: [u8; 10]) -> Self {
        ExtendedFloat(bits)
    }
    /// Stub method for macros
    #[inline]
    #[doc(hidden)]
    pub const fn to_bits(self) -> [u8; 10] {
        self.0
    }
}

extern "C" {
    pub fn extended_add(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_sub(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_mul(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_div(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_mod(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_min(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_max(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_pow(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_atan2(first: *mut ExtendedFloat, second: *const ExtendedFloat);
    pub fn extended_hypot(first: *mut ExtendedFloat, second: *const ExtendedFloat);

    pub fn extended_sqrt(first: *mut ExtendedFloat);
    pub fn extended_abs(first: *mut ExtendedFloat);
    pub fn extended_ceil(first: *mut ExtendedFloat);
    pub fn extended_floor(first: *mut ExtendedFloat);
    pub fn extended_round(first: *mut ExtendedFloat);
    pub fn extended_trunc(first: *mut ExtendedFloat);
    pub fn extended_neg(first: *mut ExtendedFloat);
    pub fn extended_exp(first: *mut ExtendedFloat);
    pub fn extended_exp_m1(first: *mut ExtendedFloat);
    pub fn extended_exp2(first: *mut ExtendedFloat);
    pub fn extended_ln(first: *mut ExtendedFloat);
    pub fn extended_ln_1p(first: *mut ExtendedFloat);
    pub fn extended_log2(first: *mut ExtendedFloat);
    pub fn extended_log10(first: *mut ExtendedFloat);
    pub fn extended_cbrt(first: *mut ExtendedFloat);
    pub fn extended_sin(first: *mut ExtendedFloat);
    pub fn extended_cos(first: *mut ExtendedFloat);
    pub fn extended_tan(first: *mut ExtendedFloat);
    pub fn extended_asin(first: *mut ExtendedFloat);
    pub fn extended_acos(first: *mut ExtendedFloat);
    pub fn extended_atan(first: *mut ExtendedFloat);
    pub fn extended_sinh(first: *mut ExtendedFloat);
    pub fn extended_cosh(first: *mut ExtendedFloat);
    pub fn extended_tanh(first: *mut ExtendedFloat);
    pub fn extended_asinh(first: *mut ExtendedFloat);
    pub fn extended_acosh(first: *mut ExtendedFloat);
    pub fn extended_atanh(first: *mut ExtendedFloat);


    pub fn extended_isfinite(first: *const ExtendedFloat) -> bool;
    pub fn extended_isnan(first: *const ExtendedFloat) -> bool;
    pub fn extended_isinf(first: *const ExtendedFloat) -> bool;
    pub fn extended_isnormal(first: *const ExtendedFloat) -> bool;
    pub fn extended_signbit(first: *const ExtendedFloat) -> c_uint;

    pub fn extended_eq(first: *const ExtendedFloat, second: *const ExtendedFloat) -> bool;
    pub fn extended_cmp(first: *const ExtendedFloat, second: *const ExtendedFloat) -> c_int;

    pub fn extended_mul_add(first: *mut ExtendedFloat, second: *const ExtendedFloat, third: *const ExtendedFloat);
    pub fn extended_modf(first: *mut ExtendedFloat, iptr: *mut ExtendedFloat);
    pub fn extended_print(first: *const ExtendedFloat, width: c_int, precision: c_int, out: *mut *mut c_char) -> c_int;
    pub fn extended_parse(out: *mut ExtendedFloat, data: *const c_char, end: *mut *mut c_char);

    pub fn extended_convert_from_f64(out: *mut ExtendedFloat, data: f64);
    pub fn extended_convert_from_f32(out: *mut ExtendedFloat, data: f32);
    pub fn extended_convert_from_i64(out: *mut ExtendedFloat, data: i64);
    pub fn extended_convert_from_u64(out: *mut ExtendedFloat, data: u64);
    pub fn extended_convert_into_f64(first: *const ExtendedFloat) -> f64;
    pub fn extended_convert_into_f32(first: *const ExtendedFloat) -> f32;
    pub fn extended_convert_into_i64(first: *const ExtendedFloat) -> i64;
    pub fn extended_convert_into_u64(first: *const ExtendedFloat) -> u64;
}