use crate::Solution;

impl Solution {
    // time limit
    // fn is_interleave_internal(s1: &[char], s2: &[char], s3: &[char]) -> bool {
    //     let mut left_index = 0;
    //     let mut right_index = 0;
    //     let mut s3_index = 0;
    //
    //     while left_index < s1.len() && right_index < s2.len() && s3_index < s3.len() {
    //         if s3[s3_index] != s1[left_index] && s3[s3_index] != s2[right_index] {
    //             return false;
    //         }
    //
    //         if s1[left_index] == s2[right_index] {
    //             return Solution::is_interleave_internal(&s1[left_index + 1..], &s2[right_index..], &s3[s3_index + 1..])
    //                 || Solution::is_interleave_internal(&s1[left_index..], &s2[right_index + 1..], &s3[s3_index + 1..]);
    //         }
    //
    //         if s1[left_index] == s3[s3_index] {
    //             left_index += 1;
    //             s3_index += 1;
    //         } else {
    //             right_index += 1;
    //             s3_index += 1;
    //         }
    //     }
    //
    //     while left_index < s1.len() && s3_index < s3.len() {
    //         if s1[left_index] == s3[s3_index] {
    //             left_index += 1;
    //             s3_index += 1;
    //         } else {
    //             return false;
    //         }
    //     }
    //
    //     while right_index < s2.len() && s3_index < s3.len() {
    //         if s2[right_index] == s3[s3_index] {
    //             right_index += 1;
    //             s3_index += 1;
    //         } else {
    //             return false;
    //         }
    //     }
    //
    //     left_index == s1.len() && right_index == s2.len() && s3_index == s3.len()
    // }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();
        let s3 = s3.chars().collect::<Vec<_>>();

        let row_bound = s1.len() + 1;
        let column_bound = s2.len() + 1;

        let mut dp = vec![vec![false; column_bound]; row_bound];
        dp[0][0] = true;

        for column in 1..column_bound {
            dp[0][column] = dp[0][column - 1] && s2[column - 1] == s3[column - 1]
        }

        for row in 1..row_bound {
            dp[row][0] = dp[row - 1][0] && s1[row - 1] == s3[row - 1]
        }

        for row in 1..row_bound {
            for column in 1..column_bound {
                let current_s3 = s3[row + column - 1];
                let s1 = s1[row - 1];
                let s2 = s2[column - 1];

                if current_s3 == s1 {
                    dp[row][column] = dp[row - 1][column]
                }
                if current_s3 == s2 {
                    dp[row][column] |= dp[row][column - 1]
                }
            }
        }

        *dp.last().unwrap().last().unwrap()
    }
}
     //   aadbbcbcac

    //    0 d b b c a
    // 0  1 0 0 0 0 0
    // a  1 0 0 0 0 0
    // a  1 1 1 1 0 0
    // b  0 1 1 1
    // c  0
    // c  0
#[test]
fn test_code_97() {
    dbg!(Solution::is_interleave("av".to_owned(), "ac".to_owned(), "avcv".to_owned()));
}