use crate::Solution;

impl Solution {
    pub fn number_of_days(year: i32, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                31
            }
            4 | 6 | 9 | 11 => { 30 }
            2 => {
                if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => {
                0
            }
        }
    }
}