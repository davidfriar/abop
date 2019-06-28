#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::{BatchSize, Bencher, Criterion};

use abop_lib::parser;

fn lsys_simple(c: &mut Criterion) {
    lsys_benchmark(
        c,
        "lsys-simple",
        r#"
            F
            F=FF
        "#,
        10,
    );
}
fn lsys_koch(c: &mut Criterion) {
    lsys_benchmark(
        c,
        "lsys-koch",
        r#"
            F-F-F-F
            F=F-F+F+FF-F-F+F
        "#,
        5,
    );
}

fn lsys_params(c: &mut Criterion) {
    lsys_benchmark(
        c,
        "lsys-params",
        r#"
            F(0.01)
            F(x)=F(x*2)+(45)F(x*2)-(60)
        "#,
        10,
    );
}

fn lsys_benchmark(c: &mut Criterion, title: &str, data: &'static str, iters: usize) {
    c.bench_function_over_inputs(
        title,
        move |b: &mut Bencher, n: &usize| {
            b.iter_batched(
                move || parser::parse_lsys(data),
                |mut lsys| lsys.nth(black_box(*n)),
                BatchSize::PerIteration,
            )
        },
        0..iters,
    );
}

criterion_group!(benches, lsys_simple, lsys_koch, lsys_params);
criterion_main!(benches);
