use std::cmp::Ordering;
use crate::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = 0i32;
        let mut left_index = 0usize;
        let mut right_index = 1usize;
        let mut sum = nums[0];

        loop {
            match target.cmp(&sum) {
                Ordering::Equal => {
                    if result == 0 {
                        result = (right_index - left_index) as i32;
                    } else {
                        result = result.min((right_index - left_index) as i32);
                    }

                    if right_index >= nums.len() {
                        break
                    }
                    sum -= nums[left_index];
                    sum += nums[right_index];

                    left_index += 1;
                    right_index += 1;
                }

                Ordering::Less => {
                    if result == 0 {
                        result = (right_index - left_index) as i32;
                    } else {
                        result = result.min((right_index - left_index) as i32);
                    }

                    sum -= nums[left_index];
                    left_index += 1;
                    if left_index >= right_index {
                        right_index += 1;
                    }
                }

                Ordering::Greater => {
                    if right_index >= nums.len() {
                        break;
                    }

                    sum += nums[right_index];
                    right_index += 1;
                }
            }
        }

        result
    }
}

#[test]
fn test_code_209() {
    // println!("{}", Solution::min_sub_array_len(4 , vec![1,4,4]));
    // println!("{}", Solution::min_sub_array_len(7 , vec![1,1,1,1,1]));
    // println!("{}", Solution::min_sub_array_len(7 , vec![2,3,1,2,4,3]));
    println!("{}", Solution::min_sub_array_len(11 , vec![1,2,3,4,5]))
}