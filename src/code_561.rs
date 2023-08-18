use crate::Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        nums.iter().step_by(2).fold(0, |left, acc| left + acc)
    }
}

#[test]
fn test_code_561() {
    println!("{}", Solution::array_pair_sum(vec![1,4,3,2]))
}