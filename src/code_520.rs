use crate::Solution;

impl Solution {

    fn letter_is_upper(char: u8) -> bool {
        char >= 'A' as u8 && char <= 'Z' as u8
    }

    pub fn detect_capital_use(word: String) -> bool {
        if word.len() < 2 {
            return true;
        }
        let word = word.as_bytes();

        let first_letter_upper_case = Solution::letter_is_upper(word[0]);
        let second_letter_upper_case = Solution::letter_is_upper(word[1]);

        let require_all_upper = if first_letter_upper_case && second_letter_upper_case {
            true
        } else if first_letter_upper_case && !second_letter_upper_case {
            false
        } else if !first_letter_upper_case && !second_letter_upper_case {
            false
        } else {
            return false;
        };

        word.iter().skip(2).all(|char| {
            let letter_is_upper = Solution::letter_is_upper(*char);
            
            letter_is_upper == require_all_upper
        })
    }
}

#[test]
fn test_code_520() {
    assert_eq!(Solution::detect_capital_use("USA".to_owned()), true);
    assert_eq!(Solution::detect_capital_use("a".to_owned()), true);
    assert_eq!(Solution::detect_capital_use("A".to_owned()), true);
    assert_eq!(Solution::detect_capital_use("Abc".to_owned()), true);
    assert_eq!(Solution::detect_capital_use("whatthehell".to_owned()), true);
    assert_eq!(Solution::detect_capital_use("aWhat".to_owned()), false);
}