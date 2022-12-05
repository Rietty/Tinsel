// Benchmarking system for my Advent of Code solutions.
#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Load library module.
#[path = "../src/library.rs"]
mod library;

// Load modules for my solutions.
#[path = "../src/day01.rs"]
mod day01;

criterion_group!(benches, day01::benchmark);

criterion_main!(benches);
