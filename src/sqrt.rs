use std::f64::NAN;
use std::num::FpCategory;
use utils::{float_encode, float_decode};

#[inline]
pub fn approximate_sqrt(target: f64) -> f64 {
    /*
     * In order to find a good initial estimate,
     * we simply divide the exponent by two.
     * This should be within sqrt(2) of the result according to a Stack Overflow answer.
     * This will allow the main algorithm to converge very quickly,
     * since it needs a good initial estimate.
     * This is very similar reasoning to the Quake fast reciprocal sqrt,
     * although slightly less complicated since we don't have to take the approximate reciprocal.
     */
    let (mantissa, exponent, sign) = float_decode(target);
    float_encode((mantissa, exponent / 2, sign))
}

pub fn sqrt(target: f64) -> f64 {
    if !target.is_nan() && !target.is_sign_positive() {
        return NAN;
    }
    match target.classify() {
        FpCategory::Nan | FpCategory::Infinite | FpCategory::Zero => {
            /*
             * This ensures that NANs propagate, sqrt(inf) == inf, and sqrt(0) == 0
             */
            return target
        },
        FpCategory::Subnormal => unimplemented!("subnormal numbers"),
        FpCategory::Normal => {},
    }
    /*
     * According to hackers delight,
     * Newton's Method is actually the most efficient
     * way to compute the sqrt
     * as it converges quadratically.
     * However, we still have to do this iteratively until the estimates aren't getting any better
     * No wonder it's such a slow operation!
     */
    let initial_estimate = approximate_sqrt(target);
    let mut last = initial_estimate;
    const HALF: f64 = 1.0/2.0;
    loop {
        let current = HALF * (last + target / last);
        if current == last {
            return current;
        }
        last = current;
    }
}

#[cfg(test)]
mod test {
    use super::sqrt;
    use utils::{assert_nearly_equals, nearly_equals};
    const ALLOWED_ERROR: f64 = 1e-10;
    #[test]
    fn basic() {
        assert_nearly_equals(sqrt(18.0), 18.0f64.sqrt(), ALLOWED_ERROR)
    }

    #[quickcheck]
    fn sqrt_matches_std(target: f64) -> bool {
        if target.is_sign_positive() {
            nearly_equals(sqrt(target), target.sqrt(), ALLOWED_ERROR)
        } else {
            true
        }
    }
}