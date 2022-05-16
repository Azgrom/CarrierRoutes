use route::Route;
use std::fmt::Debug;
use std::iter::Map;
use std::ops::{Index, IndexMut, Range};
use std::slice::{Iter, IterMut, SliceIndex};
use std::vec;
use trajectory::Trajectories;

mod route;
mod trajectory;

#[derive(Clone, Debug, PartialEq)]
struct AdjacencyMatrix {
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
            let mut from_to_relations: Vec<(usize, Option<Vec<usize>>)> = Vec::new();

            if destinations.is_empty() {
                from_to_relations = vec![(*node, None)];
            } else {
                from_to_relations = vec![(*node, Some(destinations))];
            }
            d_edges.append(&mut from_to_relations);
        }

        d_edges
    }

    fn possible_routes(src: usize, directed_edges: &Vec<(usize, Option<Vec<usize>>)>) -> Vec<Vec<usize>> {
        let mut current_route = Vec::new();
        let mut visited_nodes: Vec<usize> = Vec::new();
        let mut possible_routes: Vec<Vec<usize>> = Vec::new();

        let filtered_source = directed_edges
            .iter()
            .filter(|&d_e| src == d_e.0 && d_e.1.is_some());

        for (starting_point, endpoints) in filtered_source {
            for target in endpoints.as_ref().unwrap() {
                if visited_nodes.contains(target) {
                    continue;
                }

                visited_nodes.append(&mut vec![*starting_point, *target]);
                current_route.append(&mut vec![*starting_point, *target]);

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
    ) {
        let filtered_source = directed_edges
            .iter()
            .filter(|&d_e| *src == d_e.0 && d_e.1.is_some());

        for (_starting_point, endpoints) in filtered_source {
            for target in endpoints.as_ref().unwrap() {
                if visited_nodes.contains(target) || current_route.contains(target) {
                    break;
                }

                current_route.push(*target);

                Self::iterate_through_possibilities(
                    target,
                    current_route,
                    visited_nodes.clone(),
                    possible_routes,
                    directed_edges,
                );

                visited_nodes.push(*target);
            }
            if !current_route.is_empty() {
                possible_routes.push(current_route.to_owned());
                *current_route = visited_nodes.clone();
            }
        }
    }

    fn routes_between_two_points(dst: &usize, possible_routes: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        for route in possible_routes.iter().filter(|&routes| routes.contains(&dst)) {
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

    fn lazy_prolix_directed_dijkstra(&self, src: usize, dst: usize) -> Vec<Vec<usize>> {
        let mut matrix_nodes = self.lazy_nodes().collect::<Vec<usize>>();
        let matrix_edges = self.edges();
        let directed_edges = self.directed_edges(&matrix_nodes, &matrix_edges);

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
        let x = adj_matrix.lazy_prolix_directed_dijkstra(4, 0);

        print!("");
    }
}
