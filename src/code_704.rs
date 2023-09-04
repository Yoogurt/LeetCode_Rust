use std::cmp::Ordering;
use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut left = 0i32;
        let mut right = (nums.len() - 1) as i32;

        while left <= right {
            let middle = (left + right) >> 1;

            match target.cmp(&nums[middle as usize]) {
                Ordering::Less => {
                    right = middle - 1;
                }
                Ordering::Equal => {
                    return middle as i32;
                }
                Ordering::Greater => {
                    left = middle + 1;
                }
            }
        }

        -1
    }
}

#[test]
fn test_code_704() {
    println!("{}", Solution::search(vec![5], -5));
}