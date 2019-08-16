#[macro_use]
extern crate criterion;

use bfrs::BF;
use std::fs::File;
use std::io::{empty, sink, Read};

use criterion::{Criterion, BatchSize};

fn bench_helper(c: &mut Criterion, source_file: &'static str) {
    let mut file = File::open(source_file).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let mut bf = BF::with_sinks(sink(), empty());

    c.bench_function(source_file, move |b| {
        b.iter_batched(
            || buf.clone(),
            |buf| bf.interpret(buf),
            BatchSize::SmallInput
        );
    });
}

fn bench_square_number(c: &mut Criterion) {
    bench_helper(c, "samples/square_number.bf");
}

fn bench_bottles(c: &mut Criterion) {
    bench_helper(c, "samples/bottles.bf");
}

fn bench_quine(c: &mut Criterion) {
    bench_helper(c, "samples/quine.bf");
}

// fn big_bench_mandelbrot(c: &mut Criterion) {
//     bench_helper(c, "samples/mandelbrot.bf");
// }

fn big_bench_1(c: &mut Criterion) {
    bench_helper(c, "samples/bench_1.bf");
}

fn big_bench_2(c: &mut Criterion) {
    bench_helper(c, "samples/bench_2.bf");
}

criterion_group!(
	name = small_benches;
	config = Criterion::default();
    targets = bench_square_number, bench_bottles, bench_quine
);

criterion_group!(
	name = big_benches;
	config = Criterion::default().sample_size(10);
    targets = big_bench_1, big_bench_2
);

criterion_main!(big_benches, small_benches);
