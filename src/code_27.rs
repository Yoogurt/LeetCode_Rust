use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut index = 0;
        let mut place_index = 0;

        while index < nums.len() {
            if nums[index] != val {
                nums[place_index] = nums[index];
                place_index += 1;
            }
            index += 1;
        }

        return place_index as i32;
    }
}