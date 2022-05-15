use route::Route;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut, SliceIndex};
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

        assert_eq!(paths, vec![(0, 1, 1), (1, 2, 1), (1, 3, 1), (2, 3, 1), (4, 0, 1)]);
    }
}
