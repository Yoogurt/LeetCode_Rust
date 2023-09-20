use crate::Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        if num % 9 == 0 {
            return 9;
        }

        num - num / 9 * 9
    }
}

#[test]
fn test_code_258() {
    assert_eq!(Solution::add_digits(11), 2);
    assert_eq!(Solution::add_digits(100), 1);
    assert_eq!(Solution::add_digits(123456), 3);
    assert_eq!(Solution::add_digits(9), 9);
    assert_eq!(Solution::add_digits(81), 9)
}
