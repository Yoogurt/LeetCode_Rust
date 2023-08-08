use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut one = 0;
        let mut two = 0;

        nums.into_iter().for_each(|value| {
            let current_rob = std::cmp::max(two, one + value);
            one = two;
            two = current_rob;
        });

        two
    }
}

#[test]
fn test_code_198() {
    println!("{}", Solution::rob(vec![2, 7, 9, 3, 1]));
    println!("{}", Solution::rob(vec![2, 7,]));
    println!("{}", Solution::rob(vec![10]));
}
