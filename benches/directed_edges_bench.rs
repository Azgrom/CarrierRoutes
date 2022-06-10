use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::shortest_path;
use lib::routes::route_tuple;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Eager Dijkstra", |b| {
        b.iter(|| shortest_path(black_box(0), black_box(4), black_box(route_tuple())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
