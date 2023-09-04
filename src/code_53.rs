use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut result = i32::MIN;
        let mut current = 0;

        for value in &nums {
            current += value;

            result = result.max(current);
            current = current.max(0);
        }


        result
    }
}

#[test]
fn test_code_53() {
    println!("{}", Solution::max_sub_array(vec![ 1]))
}