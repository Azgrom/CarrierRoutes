use route::Route;
use std::collections::HashSet;
use std::fmt::Debug;

mod route;

trait AdjacencyMatrix {}

#[derive(Debug, PartialEq)]
struct Trajectories {
    routes: Vec<Route>,
}

impl Trajectories {
    pub fn new<'a>(routes: Vec<Route>) -> Result<Trajectories, &'a str> {
        let clauses = Self::guard_clauses(&routes);
        if clauses.is_some() {
            return clauses.unwrap();
        }

        Ok(Trajectories { routes })
    }

    pub fn from_routes_tuples<'a>(routes_tuple: Vec<(String, String, usize)>) -> Result<Trajectories, &'a str> {
        let routes = routes_tuple.iter().map(|r| Route::from_tuple(r).ok().unwrap()).collect();
        Self::new(routes)
    }

    pub fn add_routes<'a>(&mut self, routes: &mut Vec<Route>) -> Result<Trajectories, &'a str> {
        self.routes.append(routes);
        Self::new(self.routes.clone())
    }

    fn guard_clauses<'a, T: Debug + PartialEq>(routes: &Vec<Route>) -> Option<Result<T, &'a str>> {
        return match Self::guard_against_empty_routes::<T>(&routes) {
            Some(err) => Some(err),
            None => match Self::guard_against_repeated_routes::<T>(&routes) {
                Some(err) => Some(err),
                None => None
            },
        }
    }

    fn guard_against_empty_routes<'a, T: Debug + PartialEq>(routes: &Vec<Route>) -> Option<Result<T, &'a str>>{
        if routes.is_empty() {
            return Some(Err("There cannot be no routes"));
        }
        None
    }

    fn guard_against_repeated_routes<'a, T: Debug + PartialEq>(routes: &Vec<Route>) -> Option<Result<T, &'a str>> {
        if !Self::check_uniques(&routes) {
            return Some(Err("There cannot be repetition of a given route"));
        }
        None
    }

    fn iter(&self) -> std::slice::Iter<'_, Route> {
        self.routes.iter()
    }

    fn count_nodes(&self) -> usize {
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

        unique_nodes.len()
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

mod trajectories_tests {
    use super::*;

    const ROUTES_TUPLE: [(&str, &str, usize); 5] = [
        ("A", "B", 1),
        ("B", "C", 1),
        ("C", "D", 1),
        ("B", "D", 1),
        ("E", "A", 1),
    ];

    fn mock_routes() -> Vec<(String, String, usize)>{
        ROUTES_TUPLE.iter().map(|x| (x.0.to_string(), x.1.to_string(), x.2)).collect()
    }

    #[test]
    fn instantiation_should_fail_if_there_are_no_routes() {
        let empty_routes_array: Vec<Route> = Vec::new();
        let attempt_trajectories = Trajectories::new(empty_routes_array);
        assert_eq!(attempt_trajectories, Err("There cannot be no routes"))
    }

    #[test]
    fn instantiation_should_fail_if_there_are_repeated_routes() {
        let routes_tuple = mock_routes();
        let mut trajectories = Trajectories::from_routes_tuples(routes_tuple).ok().unwrap();

        let repeated_route = Route::new("E".to_string(), "A".to_string(), 2)
            .ok()
            .unwrap();

        let attempt_trajectories = trajectories.add_routes(&mut Vec::from([repeated_route]));

        assert_eq!(
            attempt_trajectories,
            Err("There cannot be repetition of a given route")
        )
    }

    #[test]
    fn instantiation_should_succeed_if_parameters_are_acceptable() {
        let routes_tuple = mock_routes();
        let routes = Route::from_tuple(routes_tuple.first().unwrap()).ok().unwrap();
        let routes_vec = Vec::from([routes]);
        let trajectories = Trajectories::from_routes_tuples(routes_tuple[..1].to_vec()).ok().unwrap();

        assert_eq!(trajectories, Trajectories { routes: routes_vec })
    }

    #[test]
    fn count_nodes_should_return_unique_count() {
        let routes_tuple = mock_routes();
        let trajectories = Trajectories::from_routes_tuples(routes_tuple).ok().unwrap();

        assert_eq!(trajectories.count_nodes(), 5);
    }
}
