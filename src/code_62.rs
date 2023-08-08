use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
            return 1;
        }

        let mut result = vec![vec![0; n as usize]; m as usize];

        for row in 0..m as usize {
            for column in 0..n as usize {
                if row == 0 || column == 0 {
                    result[row][column] = 1;
                    continue;
                }

                result[row][column] = result[row - 1][column] + result[row][column - 1];
            }
        }

        return *result.last().unwrap().last().unwrap();
    }
}

#[test]
fn test_code_62() {
    println!("{}", Solution::unique_paths(3, 2));
    println!("{}", Solution::unique_paths(7, 3));
}
