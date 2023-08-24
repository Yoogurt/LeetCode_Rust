use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0  {
            return 0;
        }
        if n == 1  {
            return 1;
        }

        let mut left = 0;
        let mut right = 1;

        for _ in 2..=n {
            let current = left + right;
            left = right;
            right = current;
        }

        right
    }
}

#[test]
fn test_code_509() {
    println!("{}", Solution::fib(5));
    println!("{}", Solution::fib(5));
    println!("{}", Solution::fib(5));
}