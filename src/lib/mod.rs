use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
struct Route {
    src: String,
    dst: String,
    delta: usize,
}

impl Route {
    pub fn new<'a>(src: String, dst: String, delta: usize) -> Result<Route, &'a str> {
        if src == dst {
            return Err("Impossible to have a route to itself");
        }

        if delta == 0 {
            return Err("A route with zero distance is a route to itself");
        }

        Ok(Route { src, dst, delta })
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Route to {} from {}, costing {}",
            self.src, self.dst, self.delta
        )
    }
}

#[cfg(test)]
mod route_tests {
    use super::*;

    #[test]
    fn instantiation_should_fail_if_source_and_destination_are_the_same() {
        let attempt_route = Route::new(String::from("A"), String::from("A"), 5);
        assert_eq!(attempt_route, Err("Impossible to have a route to itself"));
    }

    #[test]
    fn instantiation_should_fail_if_distance_is_zero() {
        let attempt_route = Route::new(String::from("A"), String::from("B"), 0);
        assert_eq!(
            attempt_route,
            Err("A route with zero distance is a route to itself")
        );
    }

    #[test]
    fn instantiation_should_complete_if_parameters_are_acceptable() {
        let attempt_route = Route::new(String::from("A"), String::from("B"), 1);
        assert_eq!(
            attempt_route,
            Ok(Route {
                src: "A".to_string(),
                dst: "B".to_string(),
                delta: 1
            })
        )
    }

    #[test]
    fn format_should_be_as_expected() {
        let route = Route::new(String::from("A"), String::from("B"), 1);
        assert_eq!(
            format!("{}", route.ok().unwrap()),
            "Route to A from B, costing 1"
        )
    }
}

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
