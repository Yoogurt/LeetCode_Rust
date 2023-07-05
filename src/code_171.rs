use crate::Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut pow = 1i32;

        column_title.chars().into_iter().rev().fold(0, |left, char| {
            let result = left + (char as i32 - 'A' as i32 + 1) * pow as i32;
            pow *= 26;

            result
        })
    }
}

#[test]
fn test_code_171() {
    assert_eq!(Solution::title_to_number("A".to_owned()), 1);
    assert_eq!(Solution::title_to_number("AA".to_owned()), 27);
    assert_eq!(Solution::title_to_number("AB".to_owned()), 28);
    assert_eq!(Solution::title_to_number("ZY".to_owned()), 701);
}