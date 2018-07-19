use std::f64;
use std::f64::consts::E;

use rug::{Float};
use rug::float::Special;
use rug::ops::Pow;

const PRECISION: u32 = 128;

/// Quickly computes the `floor(log2(x))` of the given number `x`
#[inline]
pub fn floor_log2(l: f64) -> i32 {
    // For some odd reason we need to 'reverse' the 'mantissa shift'
    (::utils::float_decode(l).1 as i32) + 52
}
#[inline]
fn strip_exponent(l: f64) -> f64 {
    let (significand, _, sign) = ::utils::float_decode(l);
    ::utils::float_encode((significand, -52, sign))
}

#[inline]
pub fn log2(target: f64) -> f64 {
    log2_precise(target).to_f64()
}
fn log2_precise(target: f64) -> Float {
    if target <= 0.0 || !target.is_normal() {
        return Float::with_val(PRECISION, Special::Nan)
    }
    let characteristic = floor_log2(target);
    let target = strip_exponent(target);
    let one = Float::with_val(PRECISION, 1);
    /*
     * We use the algorithm described on wikipedia: https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation
     * First we need to compute the 'characteristic' of the logarithm,
     * or rather the integer part given by `floor(log2(x))`.
     *
     */
    let mut result = Float::with_val(PRECISION, characteristic);
    let mut y = Float::with_val(PRECISION, target);
    let mut pow = one.clone();
    for _ in 0..100 {
        if y == 1.0 { break }
        debug_assert!(y >= 1.0 && y < 2.0, "Invalid y: {}", y);
        let mut z = y.clone().square();
        let mut m = 1;
        while z < 2 {
            z.square_mut();
            m += 1;
        }
        debug_assert!(z >= 2 && z < 4, "Invalid z = {}", z);
        debug_assert_nearly_equals!(
            y.clone().pow(Float::with_val(PRECISION, m).exp2()),
            z, Float::with_val(PRECISION, Float::parse("10e-6").unwrap()),
            "Invalid m = {} for y = {}", m, y
        );
        let half = z / 2;
        debug_assert!(half >= 1 && half < 2);
        if half == y {
            // We have the best approximation we can give
            break
        }
        pow *= Float::with_val(PRECISION, -m).exp2();
        result += &pow;
        y = half;
    }
    result
}

lazy_static! {
    static ref LOG2_E: Float = log2_precise(E);
    static ref LOG2_10: Float = log2_precise(10.0);
}
pub fn ln(target: f64) -> f64 {
    (log2_precise(target) / &*LOG2_E).to_f64()
}
pub fn log10(target: f64) -> f64 {
    (log2_precise(target) / &*LOG2_10).to_f64()
}
pub fn log(target: f64, base: f64) -> f64 {
    (log2_precise(target) / log2_precise(base)).to_f64()
}

#[cfg(test)]
mod test {

    use super::{log2, ln, log10, log};
    use utils::NearlyEquals;
    const ALLOWED_ERROR: f64 = 1e-15;
    #[test]
    fn basic() {
        assert_nearly_equals!(log2(18.0), 18.0f64.log2(), ALLOWED_ERROR);
        assert_eq!(log2(38.052098393873905), 5.24990410864147473);
    }

    #[quickcheck]
    fn log2_matches_std(target: f64) {
        if target.is_sign_positive() {
            assert_nearly_equals!(
                log2(target),
                target.log2(),
                ALLOWED_ERROR,
                "Invalid log2({})",
                target
            )
        }
    }
    #[quickcheck]
    fn ln_matches_std(target: f64) {
        if target.is_sign_positive() {
            assert_nearly_equals!(
                ln(target),
                target.ln(),
                ALLOWED_ERROR,
                "Invalid log2({})",
                target
            )
        }
    }

    #[quickcheck]
    fn log10_matches_std(target: f64) {
        if target.is_sign_positive() {
            assert_nearly_equals!(
                log10(target),
                target.log10(),
                ALLOWED_ERROR,
                "Invalid log10({})",
                target
            )
        }
    }

    #[quickcheck]
    fn log_matches_std(target: f64, base: f64) {
        if target.is_sign_positive() && base.is_sign_positive() {
            assert_nearly_equals!(
                log(target, base),
                target.log(base),
                ALLOWED_ERROR,
                "Invalid log({}, {})",
                target, base
            )
        }
    }
}