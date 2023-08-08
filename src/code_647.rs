use crate::Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }

        let mut mat = vec![vec![0; s.len()]; s.len()];

        let s_reserve_str = s.chars().rev().collect::<String>();
        let s_reserve = s_reserve_str.as_bytes();
        let s = s.as_bytes();

        let mut side_non_zero = 0;
        for column in 0..s.len() {
            for row in 0..s_reserve.len() {
                if s[column] == s_reserve[row] {
                    if column > 0 && row > 0 {
                        mat[column][row] = mat[column - 1][row - 1] + 1;
                    } else {
                        mat[column][row] = 1;
                    }

                    if column + row == s.len() - 1 {
                        side_non_zero += 1;
                    } else if column + row > s.len() - 1 && mat[column][row] > 1 {
                        side_non_zero += 1;
                    }
                } else {
                    mat[column][row] = 0;
                }
            }
        }

        side_non_zero as i32
    }
}

#[test]
fn test_code_647() {
    println!("{}", Solution::count_substrings("fdsklf".to_owned()));
        // println!("{}", Solution::count_substrings("aaa".to_owned()));
        // println!("{}", Solution::count_substrings("aba".to_owned()));
        println!("{}", Solution::count_substrings("aaaaa".to_owned()));
}
