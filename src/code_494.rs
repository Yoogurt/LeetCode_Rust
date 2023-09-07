use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use crate::Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = HashMap::<i32, usize>::new();

        result.insert(0,1);
        nums.into_iter().for_each(|value| {
            let mut new_result = HashMap::<i32, usize>::new();

            result.keys().for_each(|updated| {
                let can_reach_value = vec![updated - value, updated + value];
                can_reach_value.into_iter().for_each(|reached_value| {
                    let value = result.get(&updated).unwrap_or(&0);
                    new_result.insert(reached_value, new_result.get(&reached_value).unwrap_or(&0) + value);
                });
            });

            result = new_result;
        });

        *result.get(&target).unwrap_or(&0) as i32
    }
}

#[test]
fn test_code_494() {
    println!("{}", Solution::find_target_sum_ways(vec![1,1,1, 1,1], 3));
    println!("{}", Solution::find_target_sum_ways(vec![1], 1));
}