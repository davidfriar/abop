#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::{BatchSize, Bencher, Criterion};

use abop_lib::parser;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "generate",
        |b: &mut Bencher, n: &usize| {
            b.iter_batched(
                || parser::parse_lsys("F\nF=FF"),
                |mut lsys| lsys.nth(black_box(*n)),
                BatchSize::PerIteration,
            )
        },
        0..20,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
