use crate::Solution;

impl Solution {
    pub fn largest_unique_number(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1
        }

        if nums.len() <= 1 {
            return nums[0]
        }

        nums.sort();

        let mut current_index = nums.len() - 1;
        while current_index > 0 && (nums[current_index] == nums[current_index - 1]
            || (current_index < nums.len() - 1 && nums[current_index] == nums[current_index + 1])) {
            current_index -= 1;
        }

        if current_index == 0 && nums[current_index] == nums[current_index + 1] {
            -1
        } else {
            nums[current_index]
        }
    }
}

#[test]
fn test_code_1133() {
    dbg!(Solution::largest_unique_number(vec![1,1,2]));
}