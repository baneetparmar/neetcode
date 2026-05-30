impl Solution {
            pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
                    let n = points.len();

                            let mut min_dist = vec![i32::MAX; n];
                                    let mut visited = vec![false; n];

                                            min_dist[0] = 0;
                                                    let mut result = 0;

                                                            for _ in 0..n {
                                                                        let mut curr = usize::MAX;

                                                                                    for i in 0..n {
                                                                                                    if !visited[i]
                                                                                                                        && (curr == usize::MAX || min_dist[i] < min_dist[curr])
                                                                                                                                        {
                                                                                                                                                            curr = i;
                                                                                                                                                                            }
                                                                                                                                                                                        }

                                                                                                                                                                                                    visited[curr] = true;
                                                                                                                                                                                                                result += min_dist[curr];

                                                                                                                                                                                                                            let x1 = points[curr][0];
                                                                                                                                                                                                                                        let y1 = points[curr][1];

                                                                                                                                                                                                                                                    for next in 0..n {
                                                                                                                                                                                                                                                                    if !visited[next] {
                                                                                                                                                                                                                                                                                        let x2 = points[next][0];
                                                                                                                                                                                                                                                                                                            let y2 = points[next][1];

                                                                                                                                                                                                                                                                                                                                let dist = (x1 - x2).abs() + (y1 - y2).abs();

                                                                                                                                                                                                                                                                                                                                                    min_dist[next] = min_dist[next].min(dist);
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                        }

                                                                                                                                                                                                                                                                                                                                                                                                result
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                    }
