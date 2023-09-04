use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_twice: Vec<i32> = vec![];
        nums_twice.append(&mut nums.clone());
        nums_twice.append(&mut nums.clone());

        let mut map = HashMap::<i32, i32>::new();
        let mut stack: Vec<i32> = vec![];

        nums_twice.into_iter().rev().for_each(|value| {
            while !stack.is_empty() && value >= *stack.last().unwrap() {
                stack.pop();
            }

            if stack.is_empty() {
                map.insert(value, -1);
            } else {
                map.insert(value, *stack.last().unwrap());
            }

            stack.push(value);
        });

        return nums.iter().map(|value| { map[value] }).collect();
    }
}

#[test]
fn test_code_503() {
    println!("{:?}", Solution::next_greater_elements(vec![1,2,1]));
    println!("???");
}