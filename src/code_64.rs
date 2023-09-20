use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = vec![-1; grid[0].len()];

        grid.into_iter().enumerate().for_each(|(y, value)| {
            value.iter().enumerate().for_each(|(x, value)| {
                match x {
                    0 => {
                        if result[x] == -1 {
                            result[0] = *value;
                        } else {
                            result[0] = result[0] + value;
                        }
                    }
                    _ => {
                        if result[x] == -1 {
                            result[x] = result[x - 1] + value;
                        } else {
                            result[x] = (result[x - 1] + value).min(result[x] + value);
                        }
                    }
                }
            });
        });

        result.last().unwrap().clone()
    }
}

#[test]
fn test_code_64() {
    dbg!(Solution::min_path_sum([[1, 2, 3], [4, 5, 6]].into_vec()));
}