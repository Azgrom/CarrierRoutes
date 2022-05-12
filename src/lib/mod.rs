use core::fmt::{Display, Formatter};
use route::Route;

mod route;

#[derive(Debug, PartialEq)]
struct Trajectories {
    routes: Vec<Route>,
}

trait AdjacencyMatrix {}

impl Trajectories {
    pub fn new<'a>(routes: Vec<Route>) -> Result<Trajectories, &'a str> {
        if routes.is_empty() {
            return Err("There cannot be no routes");
        }

        Ok(Trajectories { routes })
    }
}

mod trajectories_tests {
    use super::*;

    #[test]
    fn instantiation_should_fail_if_there_are_no_routes() {
        let empty_routes_array: Vec<Route> = Vec::new();
        let attempt_trajectories = Trajectories::new(empty_routes_array);
        assert_eq!(attempt_trajectories, Err("There cannot be no routes"))
    }
}
