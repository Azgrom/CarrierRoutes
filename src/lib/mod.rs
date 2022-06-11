use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

pub fn adj_list(route_tuples: &[(&str, &str, usize)]) -> Vec<Vec<Edge>> {
    let mut nodes = route_tuples
        .iter()
        .fold(Vec::new(), |mut nodes: Vec<&str>, (src, dst, _)| {
            if !nodes.contains(src) {
                nodes.push(src);
            }
            if !nodes.contains(dst) {
                nodes.push(dst);
            }

            nodes
        });

    nodes.sort();

    nodes
        .iter()
        .map(|n| {
            route_tuples
                .iter()
                .filter(|rt| rt.0 == *n)
                .map(|(_, dst, delta)| Edge {
                    to: nodes.iter().position(|n| n == dst).unwrap(),
                    cost: *delta,
                })
                .collect::<Vec<Edge>>()
        })
        .collect::<Vec<Vec<Edge>>>()
}

fn guard_against_invalid_endpoints(
    src: usize,
    dst: usize,
    adj_list: &Vec<Vec<Edge>>,
) -> Result<(), String> {
    return match (src, dst, adj_list.len()) {
        (i, j, _) if i == j => Err(format!("Indexes cannot be the same! Received src = {} and dst = {}", src, dst)),
        (i, j, k) if i >= k || j >= k => Err(format!("Invalid node index equal or larger than nodes count. Received src = {}, dst = {} with nodes_count = {}", i, j, k)),
        _ => Ok(())
    };
}

pub fn eager_dijkstra(src: usize, dst: usize, adj_list: &Vec<Vec<Edge>>) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[src] = 0;
    heap.push(State { cost: 0, position: src });

    while let Some(State { cost, position }) = heap.pop() {
        if position == dst { return Some(cost); }

        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.to };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

pub fn shortest_path(src: usize, dst: usize, adj_list: Vec<Vec<Edge>>) -> Result<usize, String> {
    let routes = guard_against_invalid_endpoints(src, dst, &adj_list);
    if routes.is_err() {
        return Err(routes.unwrap_err());
    }

    if let Some(shortest_distance) = eager_dijkstra(src, dst, &adj_list) {
        Ok(shortest_distance)
    } else { Err("There were no route between endpoints".to_string()) }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.


#[cfg(test)]
mod eager_dijkstra_tests {
    use crate::{adj_list, shortest_path};

    fn test() {
        // shortest_path()
    }

    #[test]
    fn shortest_path_should_correctly_return_lesser_cost_possibility() {
        let route_tuples = [
            ("A", "B", 6),
            ("A", "E", 4),
            ("B", "A", 6),
            ("B", "C", 2),
            ("B", "D", 4),
            ("C", "B", 3),
            ("C", "D", 1),
            ("C", "E", 7),
            ("D", "B", 8),
            ("E", "B", 5),
            ("E", "D", 7),
        ];

        let adj_list = adj_list(&route_tuples);

        let mut cost = shortest_path(0, 3, adj_list.clone()).ok();
        assert_eq!(cost, Some(9));

        cost = shortest_path(3, 0, adj_list.clone()).ok();
        assert_eq!(cost, Some(14));

        cost = shortest_path(4, 0, adj_list.clone()).ok();
        assert_eq!(cost, Some(11));

        vec![
            ("A".to_string(), "B".to_string(), 1),
            ("B".to_string(), "C".to_string(), 1),
            ("C".to_string(), "D".to_string(), 1),
            ("B".to_string(), "D".to_string(), 1),
            ("E".to_string(), "A".to_string(), 1),
        ];
    }

    // #[test]
    // fn shortest_path_function_should_be_able_to_identify_no_linkage_between_endpoints() {
    //     let route_tuples = route_tuple();
    //     let cost = shortest_path(1, 4, route_tuples);
    //     assert_eq!(cost, Err("There were no route between endpoints"))
    // }
}
