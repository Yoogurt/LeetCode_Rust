use crate::Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            1 => { return 0; }
            2 => {
                return 1;
            }
            3 => {
                return 2;
            }
            _ => {}
        }

        match n % 3 {
            0 => {
                3i32.pow((n / 3) as u32)
            }
            1 => {
                4 * 3i32.pow(((n / 3) - 1) as u32)
            }
            2 => {
                2 * 3i32.pow((n / 3) as u32)
            }
            _ => {
                0
            }
        }
    }
}

#[test]
fn test_code_343() {
    println!("{}", Solution::integer_break(3));
    println!("{}", Solution::integer_break(10));
    println!("{}", Solution::integer_break(11));
    println!("{}", Solution::integer_break(12));
}