use Base::vec_string_convert::Vec2OwnedString;

use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::default();
        }

        let mut result = String::new();
        let chars = strs.iter().map(|value| value.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        if chars[0].is_empty() {
            return String::default();
        }


        chars[0].iter().enumerate().any(|(index, char)| {
            let ret = chars[1..].into_iter().any(|value| {
                if value[index] != *char {
                    true
                } else {
                    false
                }
            });

            if !ret {
                result.push(*char);
            }
            ret
        });
       
        result
    }
}

#[test]
fn test_code_14() {
    println!("{}", Solution::longest_common_prefix(vec!["asd", "asggh", "as"].to_owned_string()));
}