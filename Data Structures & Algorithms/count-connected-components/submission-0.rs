impl Solution {
        pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
                let n = n as usize;

                        // adjacency list
                                let mut graph = vec![vec![]; n];

                                        for edge in edges {
                                                    let u = edge[0] as usize;
                                                                let v = edge[1] as usize;

                                                                            graph[u].push(v);
                                                                                        graph[v].push(u);
                                                                                                }

                                                                                                        fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
                                                                                                                    visited[node] = true;

                                                                                                                                for &nei in &graph[node] {
                                                                                                                                                if !visited[nei] {
                                                                                                                                                                    dfs(nei, graph, visited);
                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                        }

                                                                                                                                                                                                                let mut visited = vec![false; n];
                                                                                                                                                                                                                        let mut components = 0;

                                                                                                                                                                                                                                for node in 0..n {
                                                                                                                                                                                                                                            if !visited[node] {
                                                                                                                                                                                                                                                            dfs(node, &graph, &mut visited);
                                                                                                                                                                                                                                                                            components += 1;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                }

                                                                                                                                                                                                                                                                                                        components
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                            }
