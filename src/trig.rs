//! Basic trigonometric functions
use std::f64;

const PRECISION: u32 = 128;

/// Returns a tuple of the sine and cosine
/// of the specified number in degrees radian.
pub fn sin_cos(x: f64) -> (f64, f64) {
    (sin(x), cos(x))
}

pub fn sin(mut x: f64) -> f64 {
    // Sine is an odd function
    if x.is_sign_negative() {
        return -sin(-x);
    }
    // Reduce to [0, 2pi]
    x %= 2.0 * ::std::f64::consts::PI;
    if x <= ::std::f64::consts::FRAC_PI_2 {
        sin0(x)
    } else if x <= ::std::f64::consts::PI {
        sin0(::std::f64::consts::PI - x)
    } else if x <= 3.0 * ::std::f64::consts::FRAC_PI_2 {
        -sin0(x - ::std::f64::consts::PI)
    } else {
        -sin0(::std::f64::consts::PI * 2.0 - x)
    }
}


pub fn cos(mut x: f64) -> f64 {
    // Cosine is an even function
    x = x.abs();
    // Reduce to [0, 2pi]
    x %= 2.0 * ::std::f64::consts::PI;
    if x <= ::std::f64::consts::FRAC_PI_2 {
        cos0(x)
    } else if x <= ::std::f64::consts::PI {
        -cos0(::std::f64::consts::PI - x)
    } else if x <= 3.0 * ::std::f64::consts::FRAC_PI_2 {
        -cos0(x - ::std::f64::consts::PI)
    } else {
        cos0(::std::f64::consts::PI * 2.0 - x)
    }
}

// Determined empirically
const APPROX_ORDER: usize = 18;
const RECIP_FACT: [f64; 22] = [
    1.0, 1.0, 0.5, 0.16666666666666666, 0.041666666666666664, 0.008333333333333333,
    0.001388888888888889, 0.0001984126984126984, 2.48015873015873e-05, 2.7557319223985893e-06,
    2.755731922398589e-07, 2.505210838544172e-08, 2.08767569878681e-09, 1.6059043836821613e-10,
    1.1470745597729725e-11, 7.647163731819816e-13, 4.779477332387385e-14, 2.8114572543455206e-15,
    1.5619206968586225e-16, 8.22063524662433e-18, 4.110317623312165e-19, 1.9572941063391263e-20
];
fn sin0(x: f64) -> f64 {
    // Taylor expansion at zero is sum of ((-1)^n)x^(2n+1))/(2n+1)!
    debug_assert!(x.is_sign_positive() && x <= ::std::f64::consts::FRAC_PI_2);
    let mut n = 0usize;
    let mut result = 0.0;
    while 2 * n + 1 < APPROX_ORDER {
        let mut term = x.powi((2 * n + 1) as i32)*RECIP_FACT[2 * n + 1];
        if n & 1 == 1 {
            term = -term;
        }
        result += term;
        n += 1;
    }
    result
}

fn cos0(x: f64) -> f64 {
    // Taylor expansion at zero is sum of ((-1)^k x^(2k))/((2k)!)
    debug_assert!(x.is_sign_positive() && x <= ::std::f64::consts::FRAC_PI_2);
    let mut n = 0usize;
    let mut result = 0.0;
    while 2 * n < APPROX_ORDER {
        let mut term = x.powi((2 * n) as i32)*RECIP_FACT[2 * n];
        if n & 1 == 1 {
            term = -term;
        }
        result += term;
        n += 1;
    }
    result
}


#[cfg(test)]
mod test {
    use super::{sin, cos, sin_cos};
    use utils::NearlyEquals;
    use std::f64::consts::FRAC_PI_4;
    const ALLOWED_ERROR: f64 = 1e-12;

    #[test]
    fn basic() {
        assert_eq!(sin(0.0), 0.0);
        assert_eq!(cos(0.0), 1.0);
        assert_nearly_equals!(
            sin(FRAC_PI_4),
            (2.0f64).sqrt() / 2.0,
            ALLOWED_ERROR
        );
        assert_nearly_equals!(
            sin_cos(1.0),
            (1.0f64).sin_cos(),
            ALLOWED_ERROR
        );
    }

    #[quickcheck]
    fn matches_std(target: f64) {
        if target.is_sign_positive() {
            assert_nearly_equals!(
                sin_cos(target),
                target.sin_cos(),
                ALLOWED_ERROR,
                "Failed {}",
                target
            )
        }
    }
}