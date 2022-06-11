use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lib::{adj_list, eager_dijkstra};

pub fn criterion_benchmark(c: &mut Criterion) {
    let route_graphs = vec![
        (
            vec![
                ("A", "B", 1),
                ("B", "C", 1),
                ("C", "D", 1),
                ("B", "D", 1),
                ("E", "A", 1),
            ],
            (4, 3),
        ),
        (
            vec![
                ("A", "B", 6),
                ("A", "E", 4),
                ("B", "A", 6),
                ("B", "D", 4),
                ("C", "B", 3),
                ("C", "D", 1),
                ("C", "E", 7),
                ("D", "C", 8),
                ("E", "A", 7),
            ],
            (4, 2),
        ),
    ];

    let mut group = c.benchmark_group("Dijkstra algorithms");
    for route_tuples in route_graphs.iter() {
        let adj_list = adj_list(&route_tuples.0);

        group.bench_with_input(
            BenchmarkId::new(
                "Eager Version",
                format!("Longest path through {} nodes", route_tuples.0.len()),
            ),
            &adj_list,
            move |b, al| {
                b.iter(|| eager_dijkstra(route_tuples.1 .0, route_tuples.1 .1, &al));
            },
        );
    }

    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
