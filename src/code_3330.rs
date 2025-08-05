use crate::Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut last_chars: Option<char> = None;
        let mut result = 1;
        for char in word.chars() {
            match last_chars {
                None => {
                    last_chars = Some(char)
                }
                Some(last_char) => {
                    if last_char == char {
                        result += 1;
                    } else {
                        last_chars = Some(char)
                    }
                }
            }
        }

        return result
    }
}

#[test]
fn test_code_3330() {
    dbg!(Solution::possible_string_count("abbcccc".to_owned()));
}