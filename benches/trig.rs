#[macro_use]
extern crate criterion;
extern crate num;

extern crate toymath;

use std::time::Duration;
use std::f64::consts::{PI, FRAC_PI_6, FRAC_PI_4, FRAC_PI_3, FRAC_PI_2};

use criterion::{Criterion, ParameterizedBenchmark};

use toymath::sin_cos;

fn bench_trig(c: &mut Criterion) {
    c.bench(
        "trig",
        ParameterizedBenchmark::new(
            "stdlib",
            |b, &f| b.iter(|| f.sin_cos()),
            vec![0.0, 0.42398409132841234, 1.48978987, FRAC_PI_6, FRAC_PI_4, FRAC_PI_3, FRAC_PI_2, PI, PI + FRAC_PI_2]
        ).with_function(
            "toymath",
            |b, &f| b.iter(|| sin_cos(f))
        )
            .warm_up_time(Duration::from_millis(1000))
            .measurement_time(Duration::from_millis(1000))
    );
}



criterion_group!(benches, bench_trig);
criterion_main!(benches);
