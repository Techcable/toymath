use rug::Float;

const PRECISION: u32 = 128;

fn big_pow(base: Float, power: Float, precision: Float) -> Float {
    // Taken from stack overflow: https://stackoverflow.com/a/3519308/5713037
    if base == 1.0 {
        Float::with_val(PRECISION, 1)
    } else if power < 0.0 {
        Float::with_val(PRECISION, 1.0) / big_pow(base, -power, precision)
    } else if power >= 10.0 {
        big_pow(base, power / 2, precision / 2).square()
    } else if power >= 1.0 {
        let f = big_pow(base.clone(), power - 1, precision);
        base * f
    } else if precision >= 1.0 {
        base.sqrt()
    } else {
        big_pow(base, power * 2, precision * 2).sqrt()
    }
}
pub fn pow(base: f64, power: f64) -> f64 {
    big_pow(
        Float::with_val(PRECISION, base),
        Float::with_val(PRECISION, power),
        Float::with_val(PRECISION, Float::parse("1e-20").unwrap())
    ).to_f64()
}
pub fn ipow(mut base: f64, mut power: i32) -> f64 {
    let recip = power < 0;
    let mut result = 1.0;
    loop {
        if (power & 1) != 0 {
            result *= base;
        }
        power /= 2;
        if power == 0 { break }
        base *= base;
    }
    if recip { 1.0 / result } else { result }
}

#[cfg(test)]
mod test {
    use super::{pow, ipow};
    use ordered_float::OrderedFloat;
    #[test]
    fn basic() {
        assert_eq!(ipow(2.0, 0), 1.0);
        assert_eq!(ipow(2.0, 2), 4.0)
    }
    #[quickcheck]
    fn powi_matches_std(base: f64, power: i32) {
        assert_eq!(
            OrderedFloat(ipow(base, power)),
            OrderedFloat(base.powi(power)),
            "Failed {}^{}",
            base, power
        )
    }
    #[quickcheck]
    fn powf_matches_std(base: f64, power: f64) {
        assert_eq!(
            OrderedFloat(pow(base, power)),
            OrderedFloat(base.powf(power)),
            "Failed {}^{}",
            base, power
        )
    }
}