use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
            let mut graph: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();

                    for ticket in tickets {
                                graph
                                                .entry(ticket[0].clone())
                                                                .or_default()
                                                                                .push(Reverse(ticket[1].clone()));
                                                                                        }

                                                                                                let mut route = Vec::new();

                                                                                                        fn dfs(
                                                                                                                    airport: String,
                                                                                                                                graph: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
                                                                                                                                            route: &mut Vec<String>,
                                                                                                                                                    ) {
                                                                                                                                                                while let Some(next) = graph
                                                                                                                                                                                .get_mut(&airport)
                                                                                                                                                                                                .and_then(|heap| heap.pop())
                                                                                                                                                                                                            {
                                                                                                                                                                                                                            dfs(next.0, graph, route);
                                                                                                                                                                                                                                        }

                                                                                                                                                                                                                                                    route.push(airport);
                                                                                                                                                                                                                                                            }

                                                                                                                                                                                                                                                                    dfs("JFK".to_string(), &mut graph, &mut route);

                                                                                                                                                                                                                                                                            route.reverse();
                                                                                                                                                                                                                                                                                    route
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        }