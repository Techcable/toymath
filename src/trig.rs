//! Basic trigonometric functions
use extended_float::consts::{PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_1_SQRT_2};
use extended_float::{ExtendedFloat};
use ordered_float::{NotNaN, OrderedFloat};
use utils::NearlyEquals;
use num_traits::Float;
use std::fmt::{self, Debug, Formatter};

use extended_float_macros::extended_float;

/// Gives the value of `sqrt(2)/2`
///
/// This is approximiately equal to `1/sqrt(2)` by the identity
/// `1/sqrt(2) = 1/sqrt(2) * sqrt(2)/sqrt(2) = sqrt(2)/2`
const FRAC_SQRT_2_2: ExtendedFloat = FRAC_1_SQRT_2;
/// The approximate value of `sqrt(3) / 2`
const FRAC_SQRT_3_2: ExtendedFloat = extended_float!("0.8660254037844386467637231707529361834714026269051903140279034897259665");

#[derive(Copy, Clone, PartialEq)]
struct KnownAngle {
    degree: ExtendedFloat,
    sin: ExtendedFloat,
    cos: ExtendedFloat
}
impl KnownAngle {
    #[cfg(test)]
    /// For testing purposes, determine what the stdlib claims the known angle is
    pub fn stdlib(degree: ExtendedFloat) -> KnownAngle {
        let (sin, cos) = degree.sin_cos();
        KnownAngle { degree, sin, cos }
    }
    #[inline]
    pub const fn new(degree: ExtendedFloat, sin: ExtendedFloat, cos: ExtendedFloat) -> KnownAngle {
        KnownAngle { degree, sin, cos }
    }
    #[inline]
    pub fn add(self, other: KnownAngle) -> KnownAngle {
        /*
         * This is simply the 'fundamental' angle addition identity.
         * sin(x + y) = sin(x)cos(y) + cos(x)sin(y)
         * cos(x + y) = cos(x)cos(y) - sin(x)sin(y)
         */
        KnownAngle {
            degree: self.degree + other.degree,
            sin: self.sin * other.cos + self.cos * other.sin,
            cos: self.cos * other.cos - self.sin * other.sin
        }
    }
    #[inline]
    pub fn sub(self, other: KnownAngle) -> KnownAngle {
        self.add(other.neg())
    }
    #[inline]
    pub fn neg(self) -> KnownAngle {
        /*
         * This is based off the fact that sine is an odd function
         * and cosine is an even function.
         * sin(-x) = -sin(x)
         * cos(-x) = cos(x)
         */
        KnownAngle {
            degree: -self.degree,
            sin: -self.sin,
            cos: self.cos
        }
    }
    pub fn half_angle(self) -> KnownAngle {
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
        KnownAngle {
            degree: self.degree * extended_float!(0.5),
            sin: (extended_float!(0.5) * (extended_float!(1.0) - self.cos)).sqrt(),
            cos: (extended_float!(0.5) * (extended_float!(1.0) + self.cos)).sqrt(),
        }
    }
}
impl NearlyEquals<ExtendedFloat> for KnownAngle {
    #[inline]
    fn nearly_equals(self, other: Self, threshold: ExtendedFloat) -> bool {
        self.degree.nearly_equals(other.degree, threshold) &&
        self.sin.nearly_equals(other.sin, threshold) &&
            self.cos.nearly_equals(other.cos, threshold)
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

const ERROR_THRESHOLD: ExtendedFloat = extended_float!("1e-20");
/// The values in the unit circle as a tuple of radian degree,
/// sine value, and cosine value
///
/// Since floating point can't exactly represent pi or square roots,
/// most of the values are actually approximations.
//noinspection RsApproxConstant
const UNIT_CIRCLE: &[KnownAngle] = &[
    KnownAngle::new(extended_float!(0), extended_float!(0), extended_float!(1)),
    KnownAngle::new(FRAC_PI_6, extended_float!(0.5), FRAC_SQRT_3_2),
    KnownAngle::new(FRAC_PI_4, FRAC_SQRT_2_2, FRAC_SQRT_2_2),
    KnownAngle::new(FRAC_PI_3, FRAC_SQRT_3_2, extended_float!(0.5)),
    KnownAngle::new(FRAC_PI_2, extended_float!(1), extended_float!(0))
];
fn find_unit_circle(expected: ExtendedFloat) -> KnownAngle {
    *UNIT_CIRCLE.iter().find(|angle| angle.degree == expected).unwrap()
}

const HALF_TABLE_CUTOFF: ExtendedFloat = ERROR_THRESHOLD;
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
    result.push(find_unit_circle(FRAC_PI_3));
    for &unit_angle in UNIT_CIRCLE.iter().filter(|angle| {
        angle.degree == FRAC_PI_4 || angle.degree == FRAC_PI_6
    }) {
        let mut angle = unit_angle;
        while angle.degree >= HALF_TABLE_CUTOFF {
            result.push(angle);
            angle = angle.half_angle();
        }
    }
    result.sort_unstable_by_key(|&angle| NotNaN::new(angle.degree).unwrap());
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
    let mut x: ExtendedFloat = From::from(x);
    let mut neg_sin = false;
    let mut neg_cos = false;
    if !x.is_finite() {
        return (f64::nan(), f64::nan());
    }
    if x.is_sign_negative() {
        x = -x;
        neg_sin = !neg_sin;
    }
    debug_assert!(x >= extended_float!(0));
    if x >= (PI * extended_float!(2)) {
        x %= PI * extended_float!(2);
    }
    debug_assert!(x - PI < PI);
    if x >= PI {
        x -= PI;
        neg_sin = !neg_sin;
    }
    debug_assert!(x < PI);
    if x >= FRAC_PI_2 {
        x = PI - x;
        neg_cos = !neg_cos;
    }
    debug_assert!(x < FRAC_PI_2);
    // First, start with the closest unit circle approximation
    let mut angle = *UNIT_CIRCLE.iter()
        .min_by_key(|&&angle| OrderedFloat((angle.degree - x).abs()))
        .unwrap();
    let half_table: &[KnownAngle] = &HALF_TABLE;
    // TODO: Can actually use a slice instead of an index
    let mut last_index = half_table.len();
    loop {
        let error = angle.degree - x;
        let abs_error = error.abs();
        if abs_error <= ERROR_THRESHOLD { break }
        /*
         * Find the maximum angle in half_table which is <= error.abs()
         * Since the corrections should be steadily decreasing,
         * we only have to search `[0, last_index)` of half_table
         */
        last_index = match half_table[..last_index].binary_search_by_key(
            &OrderedFloat(abs_error),
            |known| OrderedFloat(known.degree)
        ) {
            Ok(index) => index + 1,
            Err(index) => {
                assert_ne!(
                    index, 0,
                    "Unable to correct {} by {} with {}", angle.degree, error, last_index);
                index
            }
        };
        let correction = half_table[last_index - 1];
        eprintln!("Last index {} with err {} and correction {:?}", last_index, error, correction);
        debug_assert!(correction.degree <= abs_error);
        if error.is_sign_positive() {
            // We've overshot (angle.degree > x), so we correct by subtracting
            angle = angle.sub(correction);
        } else {
            // We've undershot (angle.degree < x), so we correct by adding
            let corrected = angle.add(correction);
            debug_assert!((corrected.degree - x) < error, "Corrected {:?} to {:?}", angle, corrected);
            angle = corrected;
        }
    }
    debug_assert!(
        (x - angle.degree).abs() < ERROR_THRESHOLD,
        "Invalid approx {:?} for {}", angle, x
    );
    if neg_sin {
        angle.sin = -angle.sin;
    }
    if neg_cos {
        angle.cos = -angle.cos;
    }
    (f64::from(angle.sin), f64::from(angle.cos))
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
    use extended_float::consts::FRAC_PI_4;
    use super::{KnownAngle, find_unit_circle, sin, cos, sin_cos, HALF_TABLE, UNIT_CIRCLE};
    use utils::NearlyEquals;
    use num_traits::Float;
    use extended_float::ExtendedFloat;
    use extended_float_macros::extended_float;
    const ALLOWED_ERROR: ExtendedFloat = extended_float!("1e-5");
    #[test]
    fn half_angle() {
        assert_nearly_equals!(
            find_unit_circle(FRAC_PI_4).half_angle(),
            KnownAngle::stdlib(FRAC_PI_4 / extended_float!(2)),
            ALLOWED_ERROR
        );
    }
    #[test]
    fn half_table() {
        for &angle in UNIT_CIRCLE {
            assert_nearly_equals!(
                (angle.sin, angle.cos),
                angle.degree.sin_cos(),
                ALLOWED_ERROR,
                "Invalid unit circle: {}", angle.degree
            )
        }
        for &angle in HALF_TABLE.iter().rev() {
            assert_nearly_equals!(
                (angle.sin, angle.cos),
                angle.degree.sin_cos(),
                ALLOWED_ERROR,
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
    fn matches_std(target: f64) -> bool {
        if target.is_sign_positive() {
            sin_cos(target) == target.sin_cos()
        } else {
            true
        }
    }
}