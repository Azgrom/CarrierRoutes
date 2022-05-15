use std::fmt::Debug;
use std::slice::Iter;
use std::collections::HashSet;
use crate::Route;

#[derive(Debug, PartialEq)]
pub struct Trajectories {
    routes: Vec<Route>,
}

const REP_ERROR: &str = "There cannot be repetition of a given route";
const NO_ROUTE_ERROR: &str = "There cannot be no routes";

impl Trajectories {
    pub fn new<'a>(routes: Vec<Route>) -> Result<Trajectories, &'a str> {
        let clauses = Self::guard_clauses(&routes);
        if clauses.is_some() {
            return clauses.unwrap();
        }

        Ok(Trajectories { routes })
    }

    pub fn from_routes_tuples<'a>(
        routes_tuple: Vec<(String, String, usize)>,
    ) -> Result<Trajectories, &'a str> {
        let routes = routes_tuple
            .iter()
            .map(|r| Route::from_tuple(r).ok().unwrap())
            .collect();
        Self::new(routes)
    }

    pub fn append<'a>(&mut self, routes: &mut Vec<Route>) -> Result<(), &str> {
        if routes.iter().any(|r| self.routes.contains(r)) {
            return Err(REP_ERROR);
        }

        Ok(self.routes.append(routes))
    }

    fn guard_clauses<'a, T: Debug + PartialEq>(routes: &Vec<Route>) -> Option<Result<T, &'a str>> {
        return match Self::guard_against_empty_routes::<T>(&routes) {
            Some(err) => Some(err),
            None => match Self::guard_against_repeated_routes::<T>(&routes) {
                Some(err) => Some(err),
                None => None,
            },
        };
    }

    fn guard_against_empty_routes<'a, T: Debug + PartialEq>(
        routes: &Vec<Route>,
    ) -> Option<Result<T, &'a str>> {
        if routes.is_empty() {
            return Some(Err(NO_ROUTE_ERROR));
        }
        None
    }

    fn guard_against_repeated_routes<'a, T: Debug + PartialEq>(
        routes: &Vec<Route>,
    ) -> Option<Result<T, &'a str>> {
        if !Self::check_uniques(&routes) {
            return Some(Err(REP_ERROR));
        }
        None
    }

    pub(crate) fn iter(&self) -> Iter<'_, Route> {
        self.routes.iter()
    }

    pub(crate) fn nodes(&self) -> Vec<String> {
        let mut unique_nodes: Vec<String> = Vec::new();

        for route in self.iter() {
            let endpoints = route.endpoints();

            if !unique_nodes.contains(&endpoints.0) {
                unique_nodes.push(endpoints.0)
            }

            if !unique_nodes.contains(&endpoints.1) {
                unique_nodes.push(endpoints.1)
            }
        }

        unique_nodes.sort();

        unique_nodes
    }

    fn routes_tuple(routes: &Vec<Route>) -> Vec<(String, String)> {
        let mut route_tuple: Vec<(String, String)> = Vec::new();

        for route in routes.iter() {
            route_tuple.push(route.endpoints());
        }

        route_tuple
    }

    fn check_uniques(routes: &Vec<Route>) -> bool {
        let mut set = HashSet::new();
        Self::routes_tuple(routes)
            .iter()
            .all(move |r| set.insert(r))
    }
}

#[cfg(test)]
mod trajectories_tests {
    use crate::{Route, Trajectories};

    fn mock_routes() -> Vec<Route> {
        let route_tuples = [
            ("A".to_string(), "B".to_string(), 1),
            ("B".to_string(), "C".to_string(), 1),
            ("C".to_string(), "D".to_string(), 1),
            ("B".to_string(), "D".to_string(), 1),
            ("E".to_string(), "A".to_string(), 1),
        ];
        route_tuples
            .iter()
            .map(|x| Route::new(x.0.clone(), x.1.clone(), x.2).ok().unwrap())
            .collect()
    }

    #[test]
    fn instantiation_should_fail_if_there_are_no_routes() {
        let empty_routes_array: Vec<Route> = Vec::new();
        let attempt_trajectories = Trajectories::new(empty_routes_array);
        assert_eq!(attempt_trajectories, Err("There cannot be no routes"))
    }

    #[test]
    fn instantiation_should_fail_if_there_are_repeated_routes() {
        let routes = mock_routes();
        let mut trajectories = Trajectories::new(routes).ok().unwrap();

        let repeated_route = Route::new("E".to_string(), "A".to_string(), 2)
            .ok()
            .unwrap();

        let result = trajectories.append(&mut Vec::from([repeated_route]));
        assert_eq!(result, Err("There cannot be repetition of a given route"))
    }

    #[test]
    fn instantiation_should_succeed_if_parameters_are_acceptable() {
        let routes = mock_routes();
        let trajectories = Trajectories::new(routes[..1].to_vec()).ok().unwrap();

        assert_eq!(
            trajectories,
            Trajectories {
                routes: routes[..1].to_vec()
            }
        )
    }

    #[test]
    fn count_nodes_should_return_unique_count() {
        let routes = mock_routes();
        let trajectories = Trajectories::new(routes).ok().unwrap();

        assert_eq!(trajectories.nodes().len(), 5);
    }
}
