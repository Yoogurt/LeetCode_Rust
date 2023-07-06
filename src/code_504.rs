use crate::Solution;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }
        
        let mut result = String::default();
        let positive = num > 0;
        num = num.abs();
        
        while num != 0 {
            result.insert(0, ('0' as u8 + (num % 7) as u8) as char);
            num /= 7;
        }

        if !positive {
            result.insert(0, '-');
        }

        result
    }
}

#[test]
fn test_code_504() {
    assert_eq!(Solution::convert_to_base7(100), "202".to_owned());
    assert_eq!(Solution::convert_to_base7(-7), "-10".to_owned())
}