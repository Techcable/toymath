#[inline]
pub fn assert_nearly_equals(first: f64, second: f64, allowed_error: f64) {
    assert!(
        nearly_equals(first, second, allowed_error),
        "assertion failed `(left == right)`,\nleft: `{}`,\nright: `{}`"
    );
}
#[inline]
pub fn nearly_equals(first: f64, second: f64, allowed_error: f64) -> bool {
    (first - second).abs() <= allowed_error
}
/// Returns the mantissa, exponent and sign as integers.
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