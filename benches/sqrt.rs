#[macro_use]
extern crate criterion;
extern crate num;

extern crate toymath;

use std::time::Duration;
use std::f64::consts::PI;

use criterion::{Criterion, ParameterizedBenchmark, Fun};

use toymath::sqrt;

fn bench_sqrt(c: &mut Criterion) {
    c.bench(
        "sqrt",
        ParameterizedBenchmark::new(
            "stdlib",
            |b, &f| b.iter(|| f.sqrt()),
            vec![1.0, 2.0, PI, 4.0, 5.0, 100.0, 172.3, 333.33, 400.0, 625.0, 666.728, 1000.0, 10e4]
        ).with_function(
            "toymath",
            |b, &f| b.iter(|| sqrt(f))
        )
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_millis(1000))
    );
}



criterion_group!(benches, bench_sqrt);
criterion_main!(benches);
