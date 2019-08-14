#[macro_use]
extern crate criterion;

use bfrs::BF;
use std::fs::File;
use std::io::{sink, empty, Sink, Empty, Read};

use criterion::Criterion;

fn bf_factory(file_name: &str) -> BF<Sink, Empty> {
    let mut file = File::open(file_name).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    BF::with_sinks(buf, sink(), empty())
}

fn bench_helper(c: &mut Criterion, source_file: &'static str) {
    c.bench_function(source_file, move |b| {
        let mut bf = bf_factory(source_file);
        b.iter(|| bf.interpret());
    });
}

fn bench_square_number(c: &mut Criterion) {
    bench_helper(c, "samples/square_number.bf");
}

fn big_bench_mandelbrot(c: &mut Criterion) {
    bench_helper(c, "samples/mandelbrot.bf");
}

fn bench_bottles(c: &mut Criterion) {
    bench_helper(c, "samples/bottles.bf");
}

criterion_group!(
    benches,
    bench_square_number,
    // big_bench_mandelbrot,
    bench_bottles
);
criterion_main!(benches);
