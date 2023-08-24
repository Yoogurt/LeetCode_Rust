use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = 1;

        let mut dynamic_result = vec![1; nums.len()];

        nums.iter().enumerate().for_each(|(index, value)| {
            for new_index in 0..index {
                let new_value = nums[new_index];

                if *value > new_value {
                    dynamic_result[index] =
                        std::cmp::max(dynamic_result[index], dynamic_result[new_index] + 1);

                    result = std::cmp::max(result, dynamic_result[index]);
                }
            }
        });

        result
    }
}

#[test]
fn test_code_lc() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    println!("{}", Solution::length_of_lis(vec![7, 7, 7, 7, 7,]));
}
