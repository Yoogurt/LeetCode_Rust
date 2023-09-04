use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();

        let mut stack: Vec<i32> = vec![];
        nums2.into_iter().rev().for_each(|value| {
            while !stack.is_empty() && value > *stack.last().unwrap() {
                stack.pop();
            }

            if stack.is_empty() {
                map.insert(value, -1);
            } else {
                map.insert(value, *stack.last().unwrap());
            }

            stack.push(value);
        });


        return nums1.into_iter().map(|value| {
            map[&value]
        }).collect();
    }
}

#[test]
fn test_code_496() {
    println!("{:?}", Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]));
}