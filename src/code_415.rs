use crate::Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut number1 = num1.as_bytes().iter().rev();
        let mut number2 = num2.as_bytes().iter().rev();

        let mut result = String::new();

        let max_len = std::cmp::max(number1.len(), number2.len());

        let mut over_10 = false;
        for _ in 0..max_len {
            let number1 = *number1.next().unwrap_or(&('0' as u8)) - ('0' as u8);
            let number2 = *number2.next().unwrap_or(&('0' as u8)) - ('0' as u8);

            let sum = number1 + number2 + if over_10 { 1 } else { 0 };
            over_10 = sum >= 10;
            result.insert(0, ((sum % 10) + '0' as u8) as char)
        }

        if over_10 {
            result.insert(0, '1');
        }
        result
    }
}

#[test]
fn test_code_415() {
    assert_eq!(Solution::add_strings("456".to_owned(), "77".to_owned()), "533".to_owned());
    assert_eq!(Solution::add_strings("0".to_owned(), "0".to_owned()), "0".to_owned());
    assert_eq!(Solution::add_strings("11".to_owned(), "123".to_owned()), "134".to_owned());
}