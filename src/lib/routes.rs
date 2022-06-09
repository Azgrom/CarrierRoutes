use std::cmp::Ordering;
use std::ops::Range;

pub fn route_tuple() -> Vec<(String, String, usize)> {
    vec![
        ("A".to_string(), "B".to_string(), 1),
        ("B".to_string(), "C".to_string(), 1),
        ("C".to_string(), "D".to_string(), 1),
        ("B".to_string(), "D".to_string(), 1),
        ("E".to_string(), "A".to_string(), 1),
    ]
}

pub(crate) struct Routes<D, N> {
    nodes: Vec<N>,
    pub(crate) distances: Vec<D>,
    sources: Vec<usize>,
    destinations: Vec<usize>,
}

#[derive(Clone)]
pub(crate) struct Edge {
    pub(crate) to: usize,
    pub(crate) cost: usize
}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Edge {
    fn serialize_edge(&self) -> (usize, usize){
        (self.to, self.cost)
    }
}

impl From<Vec<(String, String, usize)>> for Routes<usize, String>
{
    fn from(rt: Vec<(String, String, usize)>) -> Self {
        let mut nodes = rt.iter().fold(Vec::new(), |mut nodes: Vec<String>, e| {
            if !nodes.contains(&e.0) {
                nodes.push(e.0.clone());
            }
            if !nodes.contains(&e.1) {
                nodes.push(e.1.clone());
            }

            return nodes;
        });

        nodes.sort();

        Self {
            distances: rt.iter().map(|r| r.2).collect::<Vec<usize>>(),
            sources: rt
                .iter()
                .map(|r| nodes.iter().position(|n| *n == r.0).unwrap())
                .collect(),
            destinations: rt
                .iter()
                .map(|r| nodes.iter().position(|n| *n == r.1).unwrap())
                .collect(),
            nodes,
        }
    }
}

impl<D, N> Routes<D, N>
    where
        D: Clone + PartialOrd,
        N: Clone,
{
    fn get_route(&self, i: usize) -> (N, N, D) {
        (
            self.nodes[self.sources[i]].clone(),
            self.nodes[self.destinations[i]].clone(),
            self.distances[i].clone(),
        )
    }

    pub(crate) fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.sources.len()
    }
}

impl Routes<usize, String> {
    pub(crate) fn adj_list(&self) -> Vec<Vec<Edge>> {
        let n_count = Range {
            start: 0,
            end: self.nodes_count()
        };

        n_count.map(|n|
            self.sources.iter().enumerate()
                .filter(|(_i, &node)| node == n)
                .map(|(index, &node)| Edge {
                    to: self.destinations[index],
                    cost: self.distances[index]
                }).collect::<Vec<Edge>>()
        ).collect::<Vec<Vec<Edge>>>()
    }
}

#[cfg(test)]
mod routes_tests {
    use super::*;

    #[test]
    fn test() {
        let rt = route_tuple();
        let routes = Routes::from(rt);
        let adj_list = routes.adj_list();

        assert_eq!(adj_list[0].first().unwrap().serialize_edge(), (1, 1));
        assert_eq!(adj_list[1].first().unwrap().serialize_edge(), (2, 1));
        assert_eq!(adj_list[1].last().unwrap().serialize_edge(), (3, 1));
        assert_eq!(adj_list[2].last().unwrap().serialize_edge(), (3, 1));
        assert_eq!(adj_list[4].last().unwrap().serialize_edge(), (0, 1));
    }

    #[test]
    fn test_correctly_represent_nodes_edges_and_cost() {
        let rt = route_tuple();
        let routes = Routes::from(rt);

        assert_eq!(routes.get_route(0), ("A".to_string(), "B".to_string(), 1));
        assert_eq!(routes.get_route(1), ("B".to_string(), "C".to_string(), 1));
        assert_eq!(routes.get_route(2), ("C".to_string(), "D".to_string(), 1));
        assert_eq!(routes.get_route(3), ("B".to_string(), "D".to_string(), 1));
        assert_eq!(routes.get_route(4), ("E".to_string(), "A".to_string(), 1));
    }

    #[test]
    fn nodes_count_should_return_the_exact_number_of_nodes() {
        let rt = route_tuple();
        let routes = Routes::from(rt);

        assert_eq!(routes.nodes_count(), 5);
    }
}
