use std::cmp::min;
use std::iter::repeat;
use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut min_length = Vec::<i32>::with_capacity(nums.len());
        min_length.resize_with(nums.len(), ||{
            return -1;
        });
        min_length[0] = 0;

        nums.iter().enumerate().for_each(|(index, it)| {
            let mut next_index = index + 1;
            while (next_index < index + (*it as usize) + 1) && next_index < min_length.len() {
                if min_length[next_index] == -1 {
                    min_length[next_index] = min_length[index] + 1;
                } else {
                    min_length[next_index] = min(min_length[next_index], min_length[index] + 1);
                }

                next_index += 1;
            }
        });

        return *min_length.last().unwrap();
    }
}