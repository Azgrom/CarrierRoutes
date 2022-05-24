use route::Route;
use std::fmt::Debug;
use std::iter::Map;
use std::mem::swap;
use std::ops::{Index, IndexMut, Range};
use std::slice::{Iter, IterMut, SliceIndex};
use std::vec;
use trajectory::Trajectories;

mod route;
pub mod trajectory;

#[derive(Clone, Debug, PartialEq)]
pub struct AdjacencyMatrix {
    data: Vec<Vec<Option<usize>>>,
}

impl<Idx> Index<Idx> for AdjacencyMatrix
where
    Idx: SliceIndex<[Vec<Option<usize>>]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

impl<Idx> IndexMut<Idx> for AdjacencyMatrix
where
    Idx: SliceIndex<[Vec<Option<usize>>]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl AdjacencyMatrix {
    pub fn new(nodes_amount: usize) -> AdjacencyMatrix {
        let columns: Vec<Option<usize>> = vec![None; nodes_amount];
        let rows = vec![columns; nodes_amount];

        AdjacencyMatrix { data: rows }
    }

    pub fn from_trajectories(trajectories: Trajectories) -> AdjacencyMatrix {
        let nodes = trajectories.nodes();
        let mut adj_matrix = Self::new(nodes.len());

        for route in trajectories.iter() {
            let row_index = nodes.iter().position(|n| *n == route.source()).unwrap();
            let col_index = nodes
                .iter()
                .position(|n| *n == route.destination())
                .unwrap();

            adj_matrix[row_index][col_index] = Some(route.distance());
        }

        adj_matrix
    }

    pub fn iter(&self) -> Iter<'_, Vec<Option<usize>>> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, Vec<Option<usize>>> {
        self.data.iter_mut()
    }

    fn edges(&self) -> Vec<(usize, usize, usize)> {
        let mut edges: Vec<(usize, usize, usize)> = Vec::new();

        for (row_index, row) in self.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if let Some(el_weight) = col {
                    edges.push((row_index, col_index, *el_weight));
                }
            }
        }

        edges
    }

    fn lazy_nodes(&self) -> Map<Range<usize>, fn(usize) -> usize> {
        let matrix_len = self.data.len();

        (0..matrix_len).map(|x| x)
    }

    fn directed_edges(
        &self,
        matrix_nodes: &Vec<usize>,
        matrix_edges: &Vec<(usize, usize, usize)>,
    ) -> Vec<(usize, Option<Vec<usize>>)> {
        let mut d_edges: Vec<(usize, Option<Vec<usize>>)> = Vec::new();

        for node in matrix_nodes {
            let destinations = matrix_edges
                .iter()
                .filter(|&edge| *node == edge.0)
                .map(|rs| rs.1)
                .collect::<Vec<usize>>();

            if let false = destinations.is_empty() {
                d_edges.append(&mut vec![(*node, Some(destinations))]);
            } else {
                d_edges.append(&mut vec![(*node, None)]);
            }
        }

        d_edges
    }

    fn directed_edges2(
        &self,
        matrix_nodes: &Vec<usize>,
        matrix_edges: &Vec<(usize, usize, usize)>,
    ) -> Vec<(usize, Option<Vec<usize>>)> {
        let mut d_edges: Vec<(usize, Option<Vec<usize>>)> = Vec::new();

        matrix_nodes.iter().for_each(|node| {
            let destinations = matrix_edges
                .iter()
                .filter(|&edge| *node == edge.0)
                .map(|rs| rs.1)
                .collect::<Vec<usize>>();
            if let false = destinations.is_empty() {
                d_edges.append(&mut vec![(*node, Some(destinations))]);
            } else {
                d_edges.append(&mut vec![(*node, None)]);
            }
        });

        d_edges
    }

    fn possible_routes(
        src: usize,
        directed_edges: &Vec<(usize, Option<Vec<usize>>)>,
    ) -> Vec<Vec<usize>> {
        let mut current_route = Vec::new();
        let mut visited_nodes: Vec<usize> = Vec::new();
        let mut possible_routes: Vec<Vec<usize>> = Vec::new();

        let filtered_source = directed_edges
            .iter()
            .filter(|&d_e| src == d_e.0 && d_e.1.is_some());

        for (starting_point, endpoints) in filtered_source {
            current_route.push(*starting_point);
            visited_nodes.push(*starting_point);

            for target in endpoints.as_ref().unwrap() {
                if visited_nodes.contains(target) {
                    continue;
                }

                Self::iterate_through_possibilities(
                    target,
                    &mut current_route,
                    visited_nodes.clone(),
                    &mut possible_routes,
                    directed_edges,
                );
            }
        }

        possible_routes
    }

    fn iterate_through_possibilities(
        src: &usize,
        current_route: &mut Vec<usize>,
        mut visited_nodes: Vec<usize>,
        possible_routes: &mut Vec<Vec<usize>>,
        directed_edges: &Vec<(usize, Option<Vec<usize>>)>,
    ) -> Vec<usize> {
        current_route.push(*src);

        let filtered_source = directed_edges
            .iter()
            .filter(|&d_e| *src == d_e.0 && d_e.1.is_some());

        if filtered_source.clone().next() == None {
            visited_nodes.pop();
            return visited_nodes;
        } else {
            for (starting_point, endpoints) in filtered_source {
                visited_nodes.push(*starting_point);

                for target in endpoints.as_ref().unwrap() {
                    if visited_nodes.contains(target) {
                        possible_routes.push(current_route.to_owned());
                        break;
                    }

                    visited_nodes = Self::iterate_through_possibilities(
                        target,
                        current_route,
                        visited_nodes,
                        possible_routes,
                        directed_edges,
                    );
                }
                if !current_route.is_empty() {
                    possible_routes.push(current_route.to_owned());

                    *current_route = visited_nodes.clone();
                }
            }

            visited_nodes
        }
    }

    fn routes_between_two_points(dst: &usize, possible_routes: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        for route in possible_routes
            .iter()
            .filter(|&routes| routes.contains(&dst))
        {
            for (index, node) in route.iter().enumerate() {
                if *node == *dst {
                    let path = route[..=index].to_vec();
                    if let false = result.contains(&path) {
                        result.push(path);
                    }
                }
            }
        }

        result
    }

    pub fn lazy_prolix_directed_dijkstra(&self, src: usize, dst: usize) -> Vec<Vec<usize>> {
        let matrix_nodes = self.lazy_nodes().collect::<Vec<usize>>();
        let matrix_edges = self.edges();
        let directed_edges = self.directed_edges(&matrix_nodes, &matrix_edges);

        let possible_routes = Self::possible_routes(src, &directed_edges);

        let result = Self::routes_between_two_points(&dst, possible_routes);

        result
    }

    pub fn lazy_prolix_directed2_dijkstra(&self, src: usize, dst: usize) -> Vec<Vec<usize>> {
        let matrix_nodes = self.lazy_nodes().collect::<Vec<usize>>();
        let matrix_edges = self.edges();
        let directed_edges = self.directed_edges2(&matrix_nodes, &matrix_edges);

        let possible_routes = Self::possible_routes(src, &directed_edges);

        let result = Self::routes_between_two_points(&dst, possible_routes);

        result
    }
}

#[cfg(test)]
mod adj_matrix_tests {
    use crate::trajectory::route_tuple;
    use crate::{AdjacencyMatrix, Trajectories};

    fn mock_adj_matrix() -> AdjacencyMatrix {
        let route_tuples = route_tuple().to_vec();
        let trajectories = Trajectories::from_routes_tuples(route_tuples).ok().unwrap();

        AdjacencyMatrix::from_trajectories(trajectories)
    }

    #[test]
    fn new_should_instantiate_empty_adjacency_matrix() {
        let empty_adj_matrix = AdjacencyMatrix::new(5);
        assert!(empty_adj_matrix
            .iter()
            .all(|rows| rows.iter().all(|cols| cols.is_none())));
    }

    #[test]
    fn from_trajectories_should_instantiate_adjacency_matrix_from_trajectories() {
        let adj_matrix = mock_adj_matrix();

        assert_eq!(
            adj_matrix,
            AdjacencyMatrix {
                data: vec![
                    vec![None, Some(1), None, None, None],
                    vec![None, None, Some(1), Some(1), None],
                    vec![None, None, None, Some(1), None],
                    vec![None, None, None, None, None],
                    vec![Some(1), None, None, None, None]
                ]
            }
        )
    }

    #[test]
    fn edges_method_should_return_all_edges_and_their_weight() {
        let adj_matrix = mock_adj_matrix();
        let paths = adj_matrix.edges();

        assert_eq!(
            paths,
            vec![(0, 1, 1), (1, 2, 1), (1, 3, 1), (2, 3, 1), (4, 0, 1)]
        );
    }

    #[test]
    fn lazy_prolix_dijkstra_should_return_all_possible_routes_between_src_and_dst() {
        let adj_matrix = mock_adj_matrix();

        let mut result = adj_matrix.lazy_prolix_directed_dijkstra(4, 0);
        assert_eq!(result, [[4, 0]]);

        result = adj_matrix.lazy_prolix_directed_dijkstra(4, 3);
        assert_eq!(result, vec![vec![4, 0, 1, 2, 3], vec![4, 0, 1, 3]]);
    }
}

struct Edge {
    to: usize,
    cost: f64,
}

impl Edge {
    fn new(to: usize, cost: f64) -> Self {
        Self { to, cost }
    }
}

struct MinIndexedDHeap<T> {
    size: usize,
    max_number: usize,
    degree: usize,
    child: Vec<usize>,
    parent: Vec<usize>,
    position_map: Vec<usize>,
    inverse_map: Vec<usize>,
    values: Vec<Option<T>>,
}

impl<T> MinIndexedDHeap<T>
where T: Clone + PartialOrd
{
    fn new(mut degree: usize, mut max_size: usize) -> Self {
        if degree <= 2 {
            degree = 2;
        }
        if max_size <= degree + 1 {
            max_size = degree + 1;
        }

        let values: Vec<Option<T>> = Vec::with_capacity(max_size);
        let mut inverse_map = vec![0; max_size];
        let mut position_map = vec![0; max_size];
        let mut child = vec![0; max_size];
        let mut parent = vec![0; max_size];

        (0..max_size).for_each(|i| {
            parent[i] = (i - 1) / degree;
            child[i] = i * degree + 1;
        });

        Self {
            size: max_size,
            max_number: max_size,
            degree,
            child,
            parent,
            position_map,
            inverse_map,
            values,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn contains(&self, k: usize) -> bool {
        self.position_map[k] != 0
    }

    fn peek_min_key_index(&self) -> usize {
        self.inverse_map[0]
    }

    fn peek_min_value(&self) -> Option<T> {
        self.values[self.inverse_map[0]].clone()
    }

    fn poll_min_value(&mut self) -> Option<T> {
        let min_value = self.peek_min_value();
        self.delete(self.peek_min_key_index() as usize);

        min_value
    }

    fn delete(&mut self, k: usize) -> Option<T> {
        self.key_exists_or_throw(k);

        let i = self.position_map[k] as usize;
        self.size -= 1;
        self.swap(i, self.size);
        self.sink(i);
        self.swim(i);

        let value = self.values[k].clone();
        self.values[k] = None;
        self.position_map[k] = 0;
        self.inverse_map[self.size] = 0;

        value
    }

    fn sink(&mut self, mut i: usize) {
        let mut j = self.min_child(i);

        while j != 0 {
            self.swap(i, j);
            i = j;
            j = self.min_child(i);
        }
    }

    fn swim(&mut self, mut i: usize) {
        while i < self.parent[i] {
            self.swap(i, self.parent[i]);
            i = self.parent[i];
        }
    }

    fn min_child(&self, mut i: usize) -> usize {
        let mut index = 0;
        let from = self.child[i];
        let to = Self::min(self.size, from + self.degree);

        (from..to).for_each(|j| {
            if self.values[i] > self.values[j] {
                i = j;
                index = i;
            }
        });

        index
    }

    fn min(a: usize, b: usize) -> usize {
        return if a > b { b } else { a };
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.position_map[self.inverse_map[j] as usize] = i;
        self.position_map[self.inverse_map[i] as usize] = j;
        self.inverse_map.swap(i, j);
    }

    fn key_exists_or_throw(&self, k: usize) -> Result<u8, String> {
        if !self.contains(k as usize) {
            return Err(format!("Index does not exist; received: {}", k));
        }

        Ok(0)
    }
}
