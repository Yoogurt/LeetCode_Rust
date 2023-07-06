use crate::Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let fold_result = nums.iter().fold(0, |left, value| {
                if value == &0 {
                    result = std::cmp::max(result, left);
                    0
                } else {
                    left + 1
                }
            });
        result = std::cmp::max(result, fold_result);

        result
    }
}

#[test]
fn test_code_485() {
    // assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 1]), 3);
    // assert_eq!(
    //     Solution::find_max_consecutive_ones(vec![1, 1, 1, 1, 0, 1, 1, 1]),
    //     3
    // );
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 1, 0, 1, 1]),
        3
    );
}
