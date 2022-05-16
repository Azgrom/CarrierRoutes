use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lib::trajectory::{route_tuple, Trajectories};
use lib::AdjacencyMatrix;

pub fn criterion_benchmark(c: &mut Criterion) {
    let route_tuples = route_tuple().to_vec();
    let trajectories = Trajectories::from_routes_tuples(route_tuples).ok().unwrap();
    let adj_matrix = AdjacencyMatrix::from_trajectories(trajectories);

    let mut group = c.benchmark_group("Iterator vs Loop Directed Edges");

    for endpoints in [(4, 3)].iter() {
        group.bench_with_input(BenchmarkId::new("Loop", "(4, 3)"), endpoints, |b, i| {
            b.iter(|| adj_matrix.lazy_prolix_directed_dijkstra(i.0, i.1))
        });
        group.bench_with_input(BenchmarkId::new("Iterator", "(4, 3)"), endpoints, |b, i| {
            b.iter(|| adj_matrix.lazy_prolix_directed2_dijkstra(i.0, i.1))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
