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
    let mut initial_node = adj_list[src].clone();
    let mut min_ipq = MinIndexedPriorityQueue::from_vec_ref(&mut initial_node);

    let mut visited = vec![false; number_of_nodes];
    let mut dist = vec![usize::MAX; number_of_nodes];
    let mut from = src;
    dist[from] = 0;
    visited[from] = true;

    while !min_ipq.is_empty() {
        let min_value = min_ipq.poll_min_value();
        from = min_value.to;
        visited[min_value.to] = true;
        if min_value.cost < dist[min_value.to] {
            dist[min_value.to] = min_value.cost;
        }

        if min_value.cost > dist[min_value.to] {
            continue;
        }

        adj_list[min_value.to]
            .iter_mut()
            .filter(|e| !visited[e.to])
            .for_each(|e| {
                let new_dist = dist[from].saturating_add(e.cost);
                if new_dist < dist[e.to] {
                    dist[e.to] = new_dist;
                    e.cost = new_dist;

                    if let Some(key_index) = min_ipq.iter().position(|v| v.to == e.to) {
                        min_ipq.decrease(key_index, e.clone())
                    } else {
                        min_ipq.push(e.clone())
                    }
                }
            });

        if min_value.to == dst {
            return dist[dst];
        }
    }

    usize::MAX
}

pub fn shortest_path<'a>(
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
    use crate::routes::route_tuple;
    use crate::shortest_path;
    use std::cmp::min_by_key;

    #[test]
    fn shortest_path_should_correctly_return_lesser_cost_possibility() {
        let route_tuples = route_tuple();
        let cost = shortest_path(0, 2, route_tuples).ok();
        assert_eq!(cost, Some(2));

        let route_tuples = vec![
            ("A".to_string(), "B".to_string(), 6),
            ("A".to_string(), "E".to_string(), 4),
            ("B".to_string(), "A".to_string(), 6),
            ("B".to_string(), "C".to_string(), 2),
            ("B".to_string(), "D".to_string(), 4),
            ("C".to_string(), "B".to_string(), 3),
            ("C".to_string(), "D".to_string(), 1),
            ("C".to_string(), "E".to_string(), 7),
            ("D".to_string(), "B".to_string(), 8),
            ("E".to_string(), "B".to_string(), 5),
            ("E".to_string(), "D".to_string(), 7),
        ];
        let cost = shortest_path(0, 3, route_tuples).ok();
        assert_eq!(cost, Some(9));
    }
}
