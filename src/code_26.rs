use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut last_num = *nums.first().unwrap();
        let mut replace_index = 1;
        let mut index = 1;

        while index < nums.len() {
            let num = nums[index];

            if num != last_num {
                nums[replace_index] = nums[index];
                replace_index += 1;
                last_num = num;
            }

            index += 1;
        }

        return replace_index as i32;
    }
}