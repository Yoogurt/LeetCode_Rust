use crate::Solution;

impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|value| {
            value.sort();
        });

        let max_length = nums.first().unwrap_or(&Vec::new()).len();

        let mut result = 0;

        for index in 0..max_length {
            result += nums.iter().fold(0, |left, value| {
                std::cmp::max(left, value[index])
            });
        }

        result
    }
}

#[test]
fn test_code_2679() {
    assert_eq!(Solution::matrix_sum(vec![vec![7,2,1], vec![6,4,2], vec![6,5,3], vec![3,2,1]]), 15);
    assert_eq!(Solution::matrix_sum(vec![vec![1]]), 1);
}