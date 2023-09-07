use Base::vec_string_convert::Vec2OwnedString;

use crate::Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut result = 0;
        let mut remembers = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        let counts = strs
            .into_iter()
            .map(|value| {
                value.chars().fold((0, 0), |acc, value| match value {
                    '0' => (acc.0 + 1, acc.1),
                    '1' => (acc.0, acc.1 + 1),
                    _ => acc,
                })
            })
            .collect::<Vec<_>>();

        counts.into_iter().for_each(|(zero, one)| {
            if zero <= m {
                if one <= n {
                    for m_index in 0..=(m - zero) as usize {
                        for n_index in 0..=(n - one) as usize {
                            remembers[m as usize][n as usize] = remembers[m as usize][n as usize]
                                .max(remembers[m_index][n_index] + 1);
                            result = result.max(remembers[m as usize][n as usize])
                        }
                    }
                }
            }
        });

        result
    }
}

#[test]
fn test_code_474() {
    println!(
        "{}",
        Solution::find_max_form(vec!["10", "0", "1"].to_owned_string(), 1, 1)
    );
}
