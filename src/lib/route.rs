use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Route {
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

    pub fn from_tuple<'a>(route_tuple: &(String, String, usize)) -> Result<Route, &'a str> {
        Route::new(route_tuple.0.clone(), route_tuple.1.clone(), route_tuple.2)
    }

    pub fn endpoints(&self) -> (String, String) {
        (self.src.clone(), self.dst.clone())
    }

    pub fn source(&self) -> String {
        self.src.clone()
    }

    pub fn destination(&self) -> String {
        self.dst.clone()
    }

    pub fn distance(&self) -> usize {
        self.delta
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

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
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
    fn instantiation_should_succeed_if_parameters_are_acceptable() {
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
    fn instantiation_should_succeed_when_parameters_come_from_tuple() {
        let route_tuple = ("A".to_string(), "B".to_string(), 2);
        let route = Route::from_tuple(&route_tuple).ok().unwrap();
        assert_eq!(
            route,
            Route {
                src: "A".to_string(),
                dst: "B".to_string(),
                delta: 2
            }
        )
    }

    #[test]
    fn format_should_print_route_message() {
        let route = Route::new(String::from("A"), String::from("B"), 1);
        assert_eq!(
            format!("{}", route.ok().unwrap()),
            "Route to A from B, costing 1"
        )
    }

    #[test]
    fn endpoints_should_return_route_source_and_destiny() {
        let route = Route::new(String::from("A"), String::from("B"), 1)
            .ok()
            .unwrap();
        let endpoints = route.endpoints();
        assert_eq!(endpoints, ("A".to_string(), "B".to_string()))
    }
}
