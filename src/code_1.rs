use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map_to_index = HashMap::<usize, i32>::new();
        for (index, value) in nums.iter().enumerate() {
            map_to_index.insert(index,*value);
        }
        let mut nums = nums;
        nums.sort();

        let mut start_index = 0;
        let mut end_index = nums.len() - 1;

        while start_index < end_index {
            let result = nums[start_index] + nums[end_index];
            if result < target {
                start_index += 1;
            } else if result > target {
                end_index -= 1;
            } else {
                return vec![1,2];
            }
        }

        return vec![];
    }
}