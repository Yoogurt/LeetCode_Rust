use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left_index = 0usize;

        for index in 0..nums.len() {
            if nums[index] == 0 {
                continue;
            }

            if left_index == index {
                left_index += 1;
                continue;
            }

            nums[left_index] = nums[index];
            left_index += 1;
        }

        for index in left_index..nums.len() {
            nums[index] = 0;
        }
    }
}

#[test]
fn test_code_283() {
    let mut data = vec![1,0,2,34,0,6,8,0];
    Solution::move_zeroes(&mut data);
    println!("{:?}", data);
}