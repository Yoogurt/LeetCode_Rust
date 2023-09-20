use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // mirror vertical

        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }

        let y_bound = matrix.len();
        let x_bound = matrix[0].len();

        for y in 0..y_bound / 2 {
            for x in 0..x_bound {
                matrix[y][x] = matrix[y][x] ^ matrix[y_bound - y - 1][x];
                matrix[y_bound - y - 1][x] = matrix[y][x] ^ matrix[y_bound - y - 1][x];
                matrix[y][x] = matrix[y][x] ^ matrix[y_bound - y - 1][x];
            }
        }

        for y in 0..y_bound {
            for x in 0..x_bound - (y_bound - y) {
                matrix[y][x] = matrix[y][x] ^ matrix[x][y];
                matrix[x][y] = matrix[y][x] ^ matrix[x][y];
                matrix[y][x] = matrix[y][x] ^ matrix[x][y];
            }
        }
    }
}

#[test]
fn test_code_48() {
    let mut vec = [[1,2,3],[4,5,6],[7,8,9]].into_vec();
    Solution::rotate(&mut vec);
    dbg!(vec);
}