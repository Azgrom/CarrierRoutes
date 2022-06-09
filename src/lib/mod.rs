use crate::routes::{Edge, Routes};
use indexed_priority_queue::ipq::{IndexedBinaryHeap, IndexedPriorityQueue};
use indexed_priority_queue::MinIndexedPriorityQueue;

pub mod routes;

fn guard_against_invalid_endpoints(n: usize, src: usize, dst: usize) {
    if src >= n {
        panic!("Invalid node index");
    }
    if dst >= n {
        panic!("Invalid node index");
    }
}

fn eager_dijkstra(src: usize, dst: usize, rt: Routes<usize, String>) -> usize {
    let number_of_nodes = rt.nodes_count();
    let mut adj_list = rt.adj_list();
    let mut min_ipq_vec = adj_list
        .iter_mut()
        .map(|v| MinIndexedPriorityQueue::from_vec_ref(v))
        .collect::<Vec<MinIndexedPriorityQueue<Edge>>>();

    let mut from = src;
    let mut visited = vec![false; number_of_nodes];
    let mut dist = vec![usize::MAX; number_of_nodes];

    let mut step = 0;
    let mut origin = min_ipq_vec.remove(from);
    dist[from] = 0;

    while step < number_of_nodes {
        while !origin.is_empty() {
            if visited[from] {
                from = origin.poll_min_value().to;
                continue;
            }

            visited[from] = true;
            step += 1;

            origin.iter_mut().for_each(|e| {
                let new_dist = dist[from].saturating_add(e.cost);
                if new_dist < usize::MAX {
                    e.cost = new_dist;
                }
                dist[e.to] = e.cost;
            });

            let next_promising_value = origin.poll_min_value();
            from = next_promising_value.to;

            if from == dst {
                return dist[dst];
            }
        }

        origin = min_ipq_vec.remove(step - from);
    }

    5
}

pub fn reconstruct_path<'a>(
    src: usize,
    dst: usize,
    rt: Vec<(String, String, usize)>,
) -> Result<usize, &'a str> {
    let routes = Routes::from(rt);
    guard_against_invalid_endpoints(routes.nodes_count(), src, dst);

    let shortest_distance = eager_dijkstra(src, dst, routes);

    if shortest_distance < usize::MAX {
        return Ok(shortest_distance);
    }

    Err("There were no route between endpoints")
}

#[cfg(test)]
mod eager_dijkstra_tests {
    use crate::reconstruct_path;
    use crate::routes::route_tuple;
    use std::cmp::min_by_key;

    #[test]
    fn test() {
        let route_tuples = route_tuple().to_vec();
        let shortest_path = reconstruct_path(0, 2, route_tuples).ok();

        assert_eq!(shortest_path, Some(2));
    }

    #[test]
    fn test2() {
        let x = vec![Some(2), Some(-1)];
        let y = x.iter().min();

        println!("");
    }
}
