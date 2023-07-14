use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        if nums.iter().all(|it| it == &0) {
            return "0".to_owned();
        }

        nums.sort_by(|left, right| {
            let left = *left as u128;
            let right = *right as u128;
            let (mut left_bit_base_10, mut right_bit_base_10) = (10u128, 10u128);

            loop {
                let mut r#continue = false;
                if left / left_bit_base_10 > 0 {
                    left_bit_base_10 *= 10;
                    r#continue = true;
                }

                if right / right_bit_base_10 > 0 {
                    right_bit_base_10 *= 10;
                    r#continue = true;
                }

                if !r#continue {
                    break;
                }
            }

            let left_first = left * right_bit_base_10 + right;
            let right_first = right * left_bit_base_10 + left;

            right_first.cmp(&left_first)
        });

        nums.iter().map(|it| it.to_string()).collect()
    }
}

#[test]
fn test_code_179() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_owned());
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        "9534330".to_owned()
    );
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 30, 5, 9]),
        "953433030".to_owned()
    );
    assert_eq!(
        Solution::largest_number(vec![432, 43243]),
        "43243432".to_owned()
    );

    assert_eq!(
        Solution::largest_number(vec![9999998, 9999999, 9999997]),
        "999999999999989999997".to_owned()
    );
}
