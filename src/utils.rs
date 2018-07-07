use extended_float::ExtendedFloat;

use num_traits::Float;


#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! assert_eq_precise {
    ($left:expr, $right:expr) => ({
        match ($left, $right) {
            (left_val, right_val) => {
                if left_val != right_val {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:.20?}`,
 right: `{:.20?}`"#, left_val, right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => (assert_eq_precise!($left, $right));
    ($left:expr, $right:expr,$($arg:tt)+) => ({
        match ($left, $right) {
            (left_val, right_val) => {
                if left_val != right_val {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:.20?}`,
 right: `{:.20?}`: {}"#, left_val, right_val, format_args!($($arg)+))
                }
            }
        }
    })
}

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! assert_nearly_equals {
    ($left:expr, $right:expr, $threshold:expr) => ({
        match ($left, $right) {
            (left_val, right_val) => {
                if !left_val.nearly_equals(right_val, $threshold) {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:.20?}`,
 right: `{:.20?}`"#, left_val, right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $threshold:expr,) => (assert_nearly_equals($left, $right, $threshold));
    ($left:expr, $right:expr, $threshold:expr, $($arg:tt)+) => ({
        match ($left, $right) {
            (left_val, right_val) => {
                if !left_val.nearly_equals(right_val, $threshold) {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:.20?}`,
 right: `{:.20?}`: {}"#, left_val, right_val, format_args!($($arg)+))
                }
            }
        }
    })
}

#[cfg_attr(not(test), allow(dead_code))]
pub trait NearlyEquals<T>: Sized {
    fn nearly_equals(self, other: Self, threshold: T) -> bool;
}
impl NearlyEquals<f64> for f64 {
    #[inline]
    fn nearly_equals(self, other: Self, threshold: f64) -> bool {
        (self - other).abs() <= threshold
    }
}
impl NearlyEquals<ExtendedFloat> for ExtendedFloat {
    #[inline]
    fn nearly_equals(self, other: Self, threshold: ExtendedFloat) -> bool {
        (self - other).abs() <= threshold
    }
}
impl<T: Clone, A: NearlyEquals<T>, B: NearlyEquals<T>> NearlyEquals<T> for (A, B) {
    #[inline]
    fn nearly_equals(self, other: Self, threshold: T) -> bool {
        self.0.nearly_equals(other.0, threshold.clone()) &&
            self.1.nearly_equals(other.1, threshold)
    }
}
impl<'a, T, A: NearlyEquals<T> + Clone + 'a> NearlyEquals<T> for &'a A {
    #[inline]
    fn nearly_equals(self, other: &'a A, threshold: T) -> bool {
        (*self).clone().nearly_equals((*other).clone(), threshold)
    }
}

/// Returns the mantissa, exponent and sign as integers.
#[allow(dead_code)]
pub fn float_decode(target: f64) -> (u64, i16, i8) {
    // Copied from stdlib
    let bits = target.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
#[allow(dead_code)]
pub fn float_encode(target: (u64, i16, i8)) -> f64 {
    let mantissa = target.0;
    let mut exponent = target.1;
    let sign = target.2;
    // Exponent bias + mantissa shift
    exponent += 1023 + 52;
    let mut bits = if exponent == 0 {
        (mantissa >> 1) & 0xfffffffffffff
    } else {
        (mantissa & 0xfffffffffffff)
    };
    bits |= ((exponent as u64) & 0x7ff) << 52;
    bits |= ((sign != 1) as u64) << 63;
    f64::from_bits(bits)
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use super::*;
    //noinspection RsApproxConstant
    #[test]
    fn decode() {
        assert_eq!(float_encode(float_decode(5.0)), 5.0);
        assert_eq!(float_encode(float_decode(0.0)), 0.0);
        assert_eq!(float_encode(float_decode(PI)), PI);
    }
}