use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    fn set_zeroes_to_vec((x, y): (usize, usize),
                         matrix: &mut Vec<Vec<i32>>,
                         x_bound: usize,
                         y_bound: usize) {
        if x >= x_bound || y >= y_bound {
            return;
        }

        Solution::set_zeroes_to_vec((x, y + 1), matrix, x_bound, y_bound);
        Solution::set_zeroes_to_vec((x + 1, y), matrix, x_bound, y_bound);

        if matrix[y][x] == 0 {
            if y < y_bound - 1 {
                matrix[y + 1][x] = 0;
            }

            if x < x_bound - 1 {
                matrix[y][x + 1] = 0;
            }
        } else {
            if y < y_bound - 1 {
                if matrix[y + 1][x] == 0 {
                    matrix[y][x] = 0;
                }
            }

            if x < x_bound - 1 {
                if matrix[y][x + 1] == 0 {
                    matrix[y][x] = 0;
                }
            }
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let y_bound = matrix.len();
        let x_bound = matrix[0].len();

        Solution::set_zeroes_to_vec((0, 0), matrix, x_bound, y_bound);
    }
}

#[test]
fn test_code_73() {
    let mut data = [[0,1,2,0],[3,4,5,2],[1,3,1,5]].into_vec();
    Solution::set_zeroes(&mut data);
    dbg!(data);
}