use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut result = nums[0];

        nums.into_iter().fold(0, |left,acc| {
            let current = left + acc;
            result = std::cmp::max(result, current);
            std::cmp::max(0, current)
        });

        result
    }
}
#[test]
fn test_code_1617() {
    println!("{}", Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}