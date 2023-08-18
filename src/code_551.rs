use crate::Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_count = 0;
        let mut late_count = 0;

        !s.chars().any(|value| match value {
            'A' => {
                late_count = 0;
                absent_count += 1;
                if absent_count >= 2 {
                    true
                } else {
                    false
                }
            }

            'L' => {
                late_count += 1;
                if late_count >= 3 {
                    true
                } else {
                    false
                }
            }

            _ => {
                late_count = 0;
                false
            }
        })
    }
}

#[test]
fn test_code_551() {
    println!("{}", Solution::check_record("PPALLP".to_owned()));
    println!("{}", Solution::check_record("PPALLL".to_owned()))
}