//! Basic trigonometric functions
use utils::NearlyEquals;
use std::fmt::{self, Debug, Formatter};
use std::f64;

use rug::Float;
use rug::float::{Constant, Special, OrdFloat};

const PRECISION: u32 = 128;

lazy_static! {
    /// Gives the value of `sqrt(2)/2`
    static ref FRAC_SQRT_2_2: Float = Float::with_val(PRECISION, 2).sqrt() / 2;
    /// The approximate value of `sqrt(3) / 2`
    static ref FRAC_SQRT_3_2: Float = Float::with_val(PRECISION, 3).sqrt() / 2;
}

#[derive(Clone, PartialEq)]
struct KnownAngle {
    degree: Float,
    sin: Float,
    cos: Float
}
impl KnownAngle {
    #[cfg(test)]
    /// For testing purposes, determine what the stdlib claims the known angle is
    pub fn stdlib(degree: Float) -> KnownAngle {
        let (sin, cos) = degree.clone().sin_cos(Float::new(PRECISION));
        KnownAngle { degree, sin, cos }
    }
    #[inline]
    pub const fn new(degree: Float, sin: Float, cos: Float) -> KnownAngle {
        KnownAngle { degree, sin, cos }
    }
    #[inline]
    pub fn add(&self, other: &KnownAngle) -> KnownAngle {
        /*
         * This is simply the 'fundamental' angle addition identity.
         * sin(x + y) = sin(x)cos(y) + cos(x)sin(y)
         * cos(x + y) = cos(x)cos(y) - sin(x)sin(y)
         */
        KnownAngle {
            degree: Float::with_val(PRECISION, &self.degree + &other.degree),
            sin: Float::with_val(PRECISION, &self.sin * &other.cos + &self.cos * &other.sin),
            cos: Float::with_val(PRECISION, &self.cos * &other.cos - &self.sin * &other.sin)
        }
    }
    #[inline]
    pub fn sub(&self, other: &KnownAngle) -> KnownAngle {
        self.add(&other.neg())
    }
    #[inline]
    pub fn neg(&self) -> KnownAngle {
        /*
         * This is based off the fact that sine is an odd function
         * and cosine is an even function.
         * sin(-x) = -sin(x)
         * cos(-x) = cos(x)
         */
        KnownAngle {
            degree: Float::with_val(PRECISION, -&self.degree),
            sin: Float::with_val(PRECISION, -&self.sin),
            cos: self.cos.clone()
        }
    }
    pub fn half_angle(&self) -> KnownAngle {
        /*
         * The half angle property is based off the angle addition identity
         * for sine.
         * cos(x + y) = cos(x)cos(y) - sin(x)sin(y)
         * cos(x + x) = cos(x)cos(x) - sin(x)sin(x)
         * cos(2x) = cos^2(x) - sin^2(x)
         * If we set x = 1/2u
         * cos(u) = cos^2(1/2u) - sin^2(1/2u)
         * Via the pythagorean identity (sin^2(x) + cos^2(x) = 1)
         * we can solve for both `sin^2(1/2u)` and `cos^2(1/2u)`.
         * sin^2(x) = 1 - cos^2(x)
         * cos^2(x) = 1 - sin^2(x)
         * cos(u) = (1 - sin^2(1/2u)) - sin^2(1/2u)
         * cos(u) = 1 - 2*sin^2(1/2u)
         * 2*sin^2(1/2u) = 1 - cos(u)
         * sin^2(1/2u) = 1/2(1 - cos(u))
         * sin(1/2u) = sqrt(1/2(1-cos(u)))
         * cos(u) = cos^2(1/2u) - (1 - cos^2(1/2u))
         * cos(u) = cos^2(1/2u) - 1 + cos^2(1/2u)
         * cos(u) = 2cos^2(1/2u) - 1
         * 2cos^2(1/2u) = cos(u) + 1
         * cos^2(1/2u) = 1/2(1 + cos(u))
         * cos(1/2u) = sqrt(1/2(1 + cos(u)))
         */
        let one = Float::with_val(PRECISION, 1);
        KnownAngle {
            degree: Float::with_val(PRECISION, &self.degree / 2),
            sin: (Float::with_val(PRECISION, &one - &self.cos) / 2i32).sqrt(),
            cos: (Float::with_val(PRECISION, &one + &self.cos) / 2i32).sqrt(),
        }
    }
}
impl NearlyEquals<Float> for KnownAngle {
    #[inline]
    fn nearly_equals(self, other: Self, threshold: Float) -> bool {
        self.degree.nearly_equals(other.degree, threshold.clone()) &&
        self.sin.nearly_equals(other.sin, threshold.clone()) &&
            self.cos.nearly_equals(other.cos, threshold.clone())
    }
}
impl Debug for KnownAngle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // NOTE: We need to preserve precision
        if let Some(precision) = f.precision() {
            f.debug_struct("KnownAngle")
                .field("degree", &format_args!("{:.*}", precision, self.degree))
                .field("sin", &format_args!("{:.*}", precision, self.sin))
                .field("cos", &format_args!("{:.*}", precision, self.cos))
                .finish()
        } else {
            f.debug_struct("KnownAngle")
                .field("degree", &self.degree)
                .field("sin", &self.sin)
                .field("cos", &self.cos)
                .finish()
        }
    }
}


lazy_static! {
    static ref ERROR_THRESHOLD: Float = Float::with_val(PRECISION, Float::parse("1e-20").unwrap());
    static ref HALF_TABLE_CUTOFF: Float = ERROR_THRESHOLD.clone() / 10;
}
/// The values in the unit circle as a tuple of radian degree,
/// sine value, and cosine value
///
/// Since floating point can't exactly represent pi or square roots,
/// most of the values are actually approximations.
//noinspection RsApproxConstant
lazy_static! {
    static ref UNIT_CIRCLE: Vec<KnownAngle> = compute_unit_circle();
}
fn compute_unit_circle() -> Vec<KnownAngle> {
    let zero = Float::with_val(PRECISION, Special::Zero);
    let pi = Float::with_val(PRECISION, Constant::Pi);
    let one = Float::with_val(PRECISION, 1);
    let half = Float::with_val(PRECISION, 0.5);
    vec![
        KnownAngle::new(zero.clone(), zero.clone(), one.clone()),
        KnownAngle::new(pi.clone() / 6, half.clone(), FRAC_SQRT_3_2.clone()),
        KnownAngle::new(pi.clone() / 4, FRAC_SQRT_2_2.clone(), FRAC_SQRT_2_2.clone()),
        KnownAngle::new(pi.clone() / 3, FRAC_SQRT_3_2.clone(), half.clone()),
        KnownAngle::new(pi.clone() / 2, one.clone(), zero.clone())
    ]
}
fn find_unit_circle(expected: Float) -> KnownAngle {
    UNIT_CIRCLE.iter().find(|angle| angle.degree == expected).unwrap().clone()
}

lazy_static! {
    /// A precomputed values for all the half angles values of ``
    /// This is sorted by radian value,
    /// so the bigger the angle the farther up in the table.
    ///
    /// This goes down for all radian values greater than `HALF_TABLE_CUTOFF`
    // TODO: Is this table too big?
    static ref HALF_TABLE: Vec<KnownAngle> = compute_half_table();
}
fn compute_half_table() -> Vec<KnownAngle> {
    let mut result = Vec::with_capacity(128 + 64);
    let pi = Float::with_val(PRECISION, Constant::Pi);
    let frac_pi_4 = pi.clone() / 4;
    let frac_pi_3 = pi.clone() / 3;
    let frac_pi_6 = pi.clone() / 6;
    result.push(find_unit_circle(frac_pi_3));
    for unit_angle in UNIT_CIRCLE.iter().filter(|angle| {
        angle.degree == frac_pi_4 || angle.degree == frac_pi_6
    }) {
        let mut angle = unit_angle.clone();
        while angle.degree >= *HALF_TABLE_CUTOFF {
            result.push(angle.clone());
            angle = angle.half_angle();
        }
    }
    result.sort_unstable_by_key(|angle| OrdFloat::from(angle.degree.clone()));
    //eprintln!("Table {:#?}", result);
    result
}
/// Returns a tuple of the sine and cosine
/// of the specified number in degrees radian.
pub fn sin_cos(x: f64) -> (f64, f64) {
    /*
     * I'm restricting myself to using trigonometry I understand
     * in order to compute the sine of the number.
     * This means we can't use the taylor series expansion
     * which is what is normally used to approximate this function,
     * I need to resort to using the unit circle and my trigonometric identities.
     *
     * The basic strategy is to first reduce values to the range `[0, pi/2]`
     * Which can be relatively easily done since the sine and cosine
     * functions are odd and even respectively.
     * Furthermore, since it's periodic and symmetric about pi/2 we can also
     * eliminate all the positive values past `pi/2` as well.
     */
    let mut x = Float::with_val(PRECISION, x);
    let mut neg_sin = false;
    let mut neg_cos = false;
    if !x.is_finite() {
        return (f64::NAN, f64::NAN);
    }
    if x.is_sign_negative() {
        x = -x;
        neg_sin = !neg_sin;
    }
    debug_assert!(x >= Float::with_val(PRECISION, Special::Zero));
    let pi = Float::with_val(PRECISION, Constant::Pi);
    let two_pi = pi.clone() * 2i32;
    let frac_pi_2 = pi.clone() / 2;
    // TOOO: Do modulo
    while x >= two_pi {
        x -= &two_pi;
    }
    debug_assert!(x.clone() - &pi < pi);
    if x > pi {
        x = two_pi.clone() - x;
        neg_sin = !neg_sin;
    } else if x == pi {
        return (0.0, -1.0)
    }
    debug_assert!(x < pi);
    if x >= frac_pi_2 {
        x = pi.clone() - &x;
        neg_cos = !neg_cos;
    }
    debug_assert!(x < frac_pi_2);
    // First, start with the closest unit circle approximation
    let mut angle = UNIT_CIRCLE.iter()
        .min_by_key(|angle| OrdFloat::from((angle.degree.clone() - &x).abs()))
        .unwrap().clone();
    let half_table: &[KnownAngle] = &HALF_TABLE;
    // TODO: Can actually use a slice instead of an index
    let mut last_index = half_table.len();
    loop {
        let error = angle.degree.clone() - &x;
        let abs_error = error.clone().abs();
        if abs_error <= *ERROR_THRESHOLD { break }
        /*
         * Find the maximum angle in half_table which is <= error.abs()
         * Since the corrections should be steadily decreasing,
         * we only have to search `[0, last_index)` of half_table
         */
        last_index = match half_table[..last_index].binary_search_by_key(
            &abs_error.as_ord(),
            |known| known.degree.as_ord()
        ) {
            Ok(index) => index + 1,
            Err(index) => {
                assert_ne!(
                    index, 0,
                    "Unable to correct {} by {} with {}", angle.degree, error, last_index);
                index
            }
        };
        let correction = &half_table[last_index - 1];
        //eprintln!("Last index {} with err {} and correction {:?}", last_index, error, correction);
        debug_assert!(correction.degree <= abs_error);
        if error.is_sign_positive() {
            // We've overshot (angle.degree > x), so we correct by subtracting
            angle = angle.sub(correction);
        } else {
            // We've undershot (angle.degree < x), so we correct by adding
            let corrected = angle.add(correction);
            debug_assert!((corrected.degree.clone() - &x) < abs_error, "Corrected {:?} to {:?}", angle, corrected);
            angle = corrected;
        }
    }
    debug_assert!(
        (x.clone() - &angle.degree).abs() < *ERROR_THRESHOLD,
        "Invalid approx {:?} for {}", angle, x
    );
    if neg_sin {
        angle.sin = -angle.sin;
    }
    if neg_cos {
        angle.cos = -angle.cos;
    }
    (angle.sin.to_f64(), angle.cos.to_f64())
}

#[inline]
pub fn sin(x: f64) -> f64 {
    sin_cos(x).0
}

#[inline]
pub fn cos(x: f64) -> f64 {
    sin_cos(x).1
}

#[cfg(test)]
mod test {
    use super::{KnownAngle, PRECISION, find_unit_circle, sin, cos, sin_cos, HALF_TABLE, UNIT_CIRCLE};
    use utils::NearlyEquals;
    use rug::Float;
    use rug::float::{Constant};
    const ALLOWED_ERROR: f64 = 1e-5;
    #[test]
    fn half_angle() {
        let pi = Float::with_val(PRECISION, Constant::Pi);
        assert_nearly_equals!(
            find_unit_circle(pi.clone() / 4).half_angle(),
            KnownAngle::stdlib(pi.clone() / 8),
            Float::with_val(PRECISION, ALLOWED_ERROR)
        );
    }
    #[test]
    fn half_table() {
        let allowed_error = Float::with_val(PRECISION, ALLOWED_ERROR);
        for angle in UNIT_CIRCLE.iter() {
            assert_nearly_equals!(
                (angle.sin.clone(), angle.cos.clone()),
                angle.degree.clone().sin_cos(Float::new(PRECISION)),
                allowed_error.clone(),
                "Invalid unit circle: {}", angle.degree
            )
        }
        for angle in HALF_TABLE.iter().rev() {
            assert_nearly_equals!(
                (angle.sin.clone(), angle.cos.clone()),
                angle.degree.clone().sin_cos(Float::new(PRECISION)),
                allowed_error.clone(),
                "Invalid half angle: {}", angle.degree
            )
        }
    }

    #[test]
    fn basic() {
        assert_eq!(sin(0.0), 0.0);
        assert_eq!(cos(0.0), 1.0);
        assert_eq_precise!(
            sin_cos(1.0),
            (1.0f64).sin_cos(),
        );
    }

    #[quickcheck]
    fn matches_std(target: f64) {
        if target.is_sign_positive() {
            assert_eq!(
                sin_cos(target),
                target.sin_cos(),
                "Failed {}",
                target
            )
        }
    }
}