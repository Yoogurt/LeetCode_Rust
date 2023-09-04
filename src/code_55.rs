use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut max_arrive = 0;

        !nums.iter().enumerate().any(|(indexed, value)| {
            if indexed > max_arrive {
                return true;
            }

            max_arrive = max_arrive.max(indexed + *value as usize);
            false
        })
    }
}

#[test]
fn test_code_55() {
    println!("{}", Solution::can_jump(vec![4, 3, 2, 1, 0]))
}