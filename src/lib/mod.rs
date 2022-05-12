use route::Route;

mod route;

trait AdjacencyMatrix {}

#[derive(Debug, PartialEq)]
struct Trajectories {
    routes: Vec<Route>,
}

impl Trajectories {
    fn iter(&self) -> std::slice::Iter<'_, Route> {
        self.routes.iter()
    }
}

impl Trajectories {
    pub fn new<'a>(routes: Vec<Route>) -> Result<Trajectories, &'a str> {
        if routes.is_empty() {
            return Err("There cannot be no routes");
        }

        Ok(Trajectories { routes })
    }

    pub fn count_nodes(&self) -> usize {
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

    fn mock_routes() -> Vec<Route> {
        ROUTES_TUPLE
            .iter()
            .map(|x| {
                Route::new(x.0.to_string(), x.1.to_string(), x.2)
                    .ok()
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn instantiation_should_fail_if_there_are_no_routes() {
        let empty_routes_array: Vec<Route> = Vec::new();
        let attempt_trajectories = Trajectories::new(empty_routes_array);
        assert_eq!(attempt_trajectories, Err("There cannot be no routes"))
    }

    #[test]
    fn instantiation_should_succeed_if_parameters_are_acceptable() {
        let routes: Vec<Route> = mock_routes();
        let trajectories = Trajectories::new(routes.clone()).ok().unwrap();

        assert_eq!(trajectories, Trajectories { routes })
    }

    #[test]
    fn count_nodes_should_return_unique_count() {
        let routes = mock_routes();
        let trajectories = Trajectories::new(routes).ok().unwrap();

        assert_eq!(trajectories.count_nodes(), 5);
    }
}
