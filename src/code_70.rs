use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => {
                return 1;
            }
            2 => {
                return 2;
            }
            _ => {}
        };

        let mut left_2 = 1;
        let mut left_1 = 2;
        let mut result = 0;

        for _ in 3..=n {
            result = left_2 + left_1;
            left_2 = left_1;
            left_1 = result;
        }

        result
    }
}

#[test] 
fn test_code_70() {
    assert_eq!(1, Solution::climb_stairs(1));
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
    assert_eq!(5, Solution::climb_stairs(4));
    assert_eq!(8, Solution::climb_stairs(5));
}
