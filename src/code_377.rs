use crate::Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = vec![0; (target + 1) as usize];

        for index in 0..result.len() {
            nums.iter().for_each(|value| {
                if index == *value as usize {
                    result[index] += 1;
                } else if index > *value as usize {
                    result[index] += result[index - *value as usize];
                }
            });
        }

        return *result.last().unwrap_or(&0);
    }
}

#[test]
fn test_code_377() {
    println!("{:?}", Solution::combination_sum4(vec![1, 2, 3], 4));
}