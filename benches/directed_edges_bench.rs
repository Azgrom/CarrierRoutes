use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lib::reconstruct_path;
use lib::routes::route_tuple;

pub fn criterion_benchmark(c: &mut Criterion) {
    let route_tuples = route_tuple().to_vec();

    let mut group = c.benchmark_group("Eager Dijkstra bench");

    for endpoints in [(0, 2)].iter() {
        group.bench_with_input(BenchmarkId::new("Loop", "(1, 2)"), endpoints, |b, i| {
            b.iter(|| reconstruct_path(i.0, i.1, route_tuples.clone()))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
