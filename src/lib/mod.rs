use route::Route;
use std::fmt::Debug;
use std::ops::Index;
use std::slice::{Iter, SliceIndex};
use trajectory::Trajectories;

mod route;
mod trajectory;

#[derive(Debug, PartialEq)]
struct AdjacencyMatrix {
    data: Vec<Vec<Option<usize>>>,
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
            let col_index = nodes.iter().position(|n| *n == route.destination()).unwrap();

            adj_matrix.data[row_index][col_index] = Some(route.distance());
        }

        adj_matrix
    }

    pub fn iter(&self) -> Iter<'_, Vec<Option<usize>>> {
        self.data.iter()
    }
}

impl<Idx> Index<Idx> for AdjacencyMatrix
where
    Idx: Index<[Vec<Option<usize>>], Output = Vec<Option<usize>>>
        + SliceIndex<[Vec<Option<usize>>], Output = Vec<Option<usize>>>,
{
    type Output = Vec<Option<usize>>;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod adj_matrix_tests {
    use crate::{AdjacencyMatrix, Trajectories};

    #[test]
    fn new_should_instantiate_empty_adjacency_matrix() {
        let empty_adj_matrix = AdjacencyMatrix::new(5);
        assert!(empty_adj_matrix
            .iter()
            .all(|rows| rows.iter().all(|cols| cols.is_none())));
    }

    #[test]
    fn from_trajectories_should_instantiate_adjacency_matrix_from_trajectories() {
        let route_tuples = [
            ("A".to_string(), "B".to_string(), 1),
            ("B".to_string(), "C".to_string(), 1),
            ("C".to_string(), "D".to_string(), 1),
            ("B".to_string(), "D".to_string(), 1),
            ("E".to_string(), "A".to_string(), 1),
        ]
        .to_vec();
        let trajectories = Trajectories::from_routes_tuples(route_tuples).ok().unwrap();
        let adj_matrix = AdjacencyMatrix::from_trajectories(trajectories);

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
}
