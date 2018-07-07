#![feature(proc_macro, proc_macro_non_items)]
extern crate extended_float;
extern crate extended_float_macros;

extern crate num_traits;

use num_traits::Float;
use extended_float::consts::{PI, FRAC_PI_2};
use extended_float::ExtendedFloat;
use extended_float_macros::extended_float;

macro_rules! assert_eq_precise {
    ($first:expr, $second:expr) => (assert_eq_precise!($first, $second, 20));
    ($first:expr, $second:expr, $precision:expr) => {{
        let left = $first;
        let right = $second;
        if left != right {
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:.precision$?}`,
 right: `{:.precision$?}`"#, left, right, precision = $precision)
        }
    }};
}
macro_rules! assert_nan {
    ($target:expr) => {{
        let value = $target;
        assert!(value.is_nan(), "Expected nan, but got {:?}", value);
    }}
}

#[test]
fn add() {
    assert_eq!(extended_float!(1) + extended_float!(2), extended_float!(3));
    assert_eq!(extended_float!(1.5) + extended_float!(2.5), extended_float!(4));
    assert_nan!(extended_float!("nan") + extended_float!(1));
}

#[test]
fn sub() {
    assert_eq!(extended_float!(2) - extended_float!(1), extended_float!(1));
    assert_eq!(extended_float!(1) - extended_float!(2), extended_float!(-1));
    assert_eq!(extended_float!(1.5) - extended_float!(2), extended_float!(-0.5));
    assert_nan!(extended_float!("nan") - extended_float!(1));
}

#[test]
fn mul() {
    assert_eq!(extended_float!(2) * extended_float!(1), extended_float!(2));
    assert_eq!(extended_float!(7) * extended_float!(7), extended_float!(49));
    assert_eq!(extended_float!(1.5) * extended_float!(2), extended_float!(3));
    assert_nan!(extended_float!("nan") * extended_float!(1));
}


#[test]
fn div() {
    assert_eq!(extended_float!(2) / extended_float!(1), extended_float!(2));
    assert_eq!(extended_float!(7) / extended_float!(2), extended_float!(3.5));
    assert_eq!(extended_float!(1.5) / extended_float!(0.5), extended_float!(3));
    assert_nan!(extended_float!("nan") / extended_float!(1));
    assert_eq!(extended_float!(1) / extended_float!(0.0), extended_float!("inf"));
    assert_eq!(extended_float!(1) / extended_float!(-0.0), extended_float!("-inf"));
}

#[test]
fn neg() {
    assert_eq!(-extended_float!(-0.0), extended_float!(0.0));
    assert_eq!(-extended_float!(25), extended_float!(-25));
    assert_eq!(-extended_float!(-25), extended_float!(25));
    assert_eq!(-extended_float!(-0.5), extended_float!(0.5));
    assert_eq!(-extended_float!(0.5), extended_float!(-0.5));
}

#[test]
fn sqrt() {
    assert_eq!(extended_float!(25).sqrt(), extended_float!(5));
    assert_eq!(extended_float!(36).sqrt(), extended_float!(6));
    assert_nan!(extended_float!(-1).sqrt());
}

#[test]
fn abs() {
    assert_eq!(extended_float!(-0.0).abs(), extended_float!(0.0));
    assert_eq!(extended_float!(25).abs(), extended_float!(25));
    assert_eq!(extended_float!(-25).abs(), extended_float!(25));
    assert_eq!(extended_float!(-0.5).abs(), extended_float!(0.5));
    assert_nan!(extended_float!("nan").abs());
}


#[test]
fn ceil() {
    assert_eq!(extended_float!(-0.0).ceil(), extended_float!(-0.0));
    assert_eq!(extended_float!(25).ceil(), extended_float!(25));
    assert_eq!(extended_float!(-25).ceil(), extended_float!(-25));
    assert_eq!(extended_float!(-0.5).ceil(), extended_float!(0));
    assert_eq!(extended_float!(0.5).ceil(), extended_float!(1));
    assert_nan!(extended_float!("nan").abs());
}



#[test]
fn floor() {
    assert_eq!(extended_float!(-0.0).floor(), extended_float!(-0.0));
    assert_eq!(extended_float!(25).floor(), extended_float!(25));
    assert_eq!(extended_float!(-25).floor(), extended_float!(-25));
    assert_eq!(extended_float!(-0.5).floor(), extended_float!(-1));
    assert_eq!(extended_float!(0.5).floor(), extended_float!(0));
    assert_nan!(extended_float!("nan").abs());
}

#[test]
fn round() {
    assert_eq!(extended_float!(-0.0).round(), extended_float!(-0.0));
    assert_eq!(extended_float!(25).round(), extended_float!(25));
    assert_eq!(extended_float!(-25).round(), extended_float!(-25));
    assert_eq!(extended_float!(-0.5).round(), extended_float!(-1));
    assert_eq!(extended_float!(0.5).ceil(), extended_float!(1));
    assert_nan!(extended_float!("nan").abs());
}

#[test]
fn trunc() {
    assert_eq!(extended_float!(-0.0).trunc(), extended_float!(-0.0));
    assert_eq!(extended_float!(25).trunc(), extended_float!(25));
    assert_eq!(extended_float!(-25).trunc(), extended_float!(-25));
    assert_eq!(extended_float!(-0.5).trunc(), extended_float!(0));
    assert_eq!(extended_float!(0.5).trunc(), extended_float!(0));
    assert_nan!(extended_float!("nan").abs());
}

#[test]
fn exp() {
    assert_eq!(extended_float!(-0.0).exp(), extended_float!(1));
    assert_eq_precise!(extended_float!(25).exp(), extended_float!("72004899337.385872528"));
    assert_eq_precise!(extended_float!(-25).exp(), extended_float!("1.3887943864964020595e-11"));
    assert_eq_precise!(extended_float!(-0.5).exp(), extended_float!("0.60653065971263342361"));
    assert_eq_precise!(extended_float!(0.5).exp(), extended_float!("1.6487212707001281469"));
}

#[test]
fn sin() {
    assert_eq!(extended_float!(0).sin(), extended_float!(0));
    assert_eq!(FRAC_PI_2.sin(), extended_float!(1));
    assert_eq_precise!(PI.sin(), extended_float!("-5.0165576126683320235e-20"));
}

#[test]
fn cos() {
    assert_eq!(extended_float!(0).cos(), extended_float!(1));
    assert_eq_precise!(FRAC_PI_2.cos(), extended_float!("-2.5082788063341660117e-20"));
    assert_eq!(PI.cos(), extended_float!(-1));
}

#[test]
fn asin() {
    assert_eq!(extended_float!(0).asin(), extended_float!(0));
    assert_eq!(extended_float!(1).asin(), FRAC_PI_2);
    assert_eq!(extended_float!(-1).asin(), -FRAC_PI_2);

}

#[test]
fn acos() {
    assert_eq!(extended_float!(0).acos(), FRAC_PI_2);
    assert_eq!(extended_float!(1).acos(), extended_float!(0));
    assert_eq!(extended_float!(-1).acos(), PI);
}

#[test]
fn eq() {
    assert_eq!(extended_float!(1.5), extended_float!(1.5));
    assert_ne!(extended_float!(1.5), extended_float!(2));
    assert_ne!(extended_float!("nan"), extended_float!(1.5));
    assert_ne!(extended_float!("nan"), extended_float!("nan"));
    assert_eq!(extended_float!("inf"), extended_float!("inf"));
}

