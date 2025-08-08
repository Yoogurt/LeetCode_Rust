use crate::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_result = 0;
        let mut current_result = 0;

        for num in nums {
            if num == max_result {
                current_result += 1;
                result = result.max(current_result);
            } else if num > max_result {
                result = 1;
                current_result = 1;
                max_result = num;
            } else {
                current_result = 0;
            }
        }

        return result;
    }
}

#[test]
fn test_code_2419() {
    dbg!(Solution::longest_subarray(vec![1, 2, 3, 4]));
    dbg!(Solution::longest_subarray(vec![1, 2, 3, 3, 4,2, 1]));
    dbg!(Solution::longest_subarray(vec![
        311155, 311155, 311155, 311155, 311155, 311155, 311155, 311155, 201191, 311155
    ]));
}
