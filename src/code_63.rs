use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut result = vec![0; obstacle_grid[0].len()];
        let mut left = 1;

        obstacle_grid.into_iter().enumerate().for_each(|(y, value)| {
            value.iter().enumerate().for_each(|(x, value)| {
                if x == 0 {
                    if *value != 0 {
                        left = 0;
                    } else if y == 0 {
                        left = 1;
                    } else {
                        left = result[0];
                    }

                    result[0] = left;
                    return;
                }

                match value {
                    0 => {
                        result[x] = result[x] + left;
                        left = result[x];
                    }
                    1 => {
                        result[x] = 0;
                        left = 0;
                    }
                    _ => {}
                }
            });
        });

        result.last().unwrap().clone()
    }
}

#[test]
fn test_code_63() {
    dbg!(Solution::unique_paths_with_obstacles(
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]].into_vec()));
}