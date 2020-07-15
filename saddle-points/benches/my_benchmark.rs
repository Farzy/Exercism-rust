use criterion::{black_box, criterion_group, criterion_main, Criterion};
use saddle_points::*;

pub fn benchmark_farzad(c: &mut Criterion) {
    let input = create_saddle((0, 1000), (0, 1000));
    c.bench_function(
        "Farzad Saddle 1000",
        |b| b.iter(|| find_saddle_points(black_box(&input))));
}

pub fn benchmark_seungha_kim(c: &mut Criterion) {
    let input = create_saddle((0, 1000), (0, 1000));
    c.bench_function(
        "seungha-kim Saddle 1000",
        |b| b.iter(|| find_saddle_points_seungha_kim(black_box(&input))));
}

pub fn benchmark_akiekintveld(c: &mut Criterion) {
    let input = create_saddle((0, 1000), (0, 1000));
    c.bench_function(
        "akiekintveld Saddle 1000",
        |b| b.iter(|| find_saddle_points_akiekintveld(black_box(&input))));
}

pub fn benchmark_amdrwe(c: &mut Criterion) {
    let input = create_saddle((0, 1000), (0, 1000));
    c.bench_function(
        "amdrwe Saddle 1000",
        |b| b.iter(|| find_saddle_points_amdrwe(black_box(&input))));
}

criterion_group!(benches, benchmark_farzad,
                 benchmark_seungha_kim, benchmark_akiekintveld,
                 benchmark_amdrwe);
criterion_main!(benches);
