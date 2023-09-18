use crate::Solution;

impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![lower, upper]];
        }

        let mut search_index = lower;
        let mut missing_area = Vec::new();

        let mut current_index = 0usize;

        while current_index < nums.len() && search_index < upper {
            if search_index != nums[current_index] {
                if current_index < nums.len() {
                    missing_area.push(vec![search_index, nums[current_index] - 1]);
                    search_index = nums[current_index] + 1;
                    current_index += 1;
                } else {
                    missing_area.push(vec![search_index, upper]);
                    break;
                }
            } else {
                search_index += 1;
                current_index += 1;
            }
        }

        if search_index < upper {
            missing_area.push(vec![search_index, upper]);
        }

        missing_area
    }
}

#[test]
fn test_code_163() {
    dbg!(Solution::find_missing_ranges(vec![-1], -2, -1));
}