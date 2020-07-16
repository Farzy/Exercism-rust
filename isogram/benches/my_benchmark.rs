use criterion::{black_box, criterion_group, criterion_main, Criterion};
use isogram::*;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::char;

pub fn benchmark_farzad(c: &mut Criterion) {
    let input = "subdermatoglyphic";
    c.bench_function(
        "Farzad Isogram",
        |b| b.iter(|| check(black_box(input))));
}

pub fn benchmark_farzad_long(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
    let source = (32..=127)
        .map(|i| char::from_u32(i).unwrap()).collect::<Vec<_>>();
    let input = source
        .choose_multiple(&mut rng, 60)
        .collect::<String>();
    c.bench_function(
        "Farzad Isogram random",
        |b| b.iter(|| {
            check(&input);
        }));
}

pub fn benchmark_hansrodtang(c: &mut Criterion) {
    let input = "subdermatoglyphic";
    c.bench_function(
        "hansrodtang Isogram",
        |b| b.iter(|| check_hansrodtang(black_box(input))));
}

pub fn benchmark_hansrodtang_long(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
    let source = (32..=127)
        .map(|i| char::from_u32(i).unwrap()).collect::<Vec<_>>();
    let input = source
        .choose_multiple(&mut rng, 60)
        .collect::<String>();
    c.bench_function(
        "hansrodtang Isogram random",
        |b| b.iter(|| {
            check_hansrodtang(&input);
        }));
}

criterion_group!(benches,
                 benchmark_farzad,
                 benchmark_farzad_long,
                 benchmark_hansrodtang,
                 benchmark_hansrodtang_long
                 );
criterion_main!(benches);
