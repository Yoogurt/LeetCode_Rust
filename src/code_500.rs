use crate::Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        fn char_shift(char: u8) -> i32 {
            return 1 << (char - 'a' as u8);
        }

        let line_one = "qwertyuiop"
            .as_bytes()
            .iter()
            .fold(0i32, |acc, char| acc | char_shift(*char));
        let line_two = "asdfghjkl"
            .as_bytes()
            .iter()
            .fold(0i32, |acc, char| acc | char_shift(*char));
        let line_three = "zxcvbnm"
            .as_bytes()
            .iter()
            .fold(0i32, |acc, char| acc | char_shift(*char));

        fn to_upper(char: u8) -> u8 {
            if char <= 'Z' as u8 {
                return char - 'A' as u8 + 'a' as u8;
            }

            return char;
        }

        words
            .into_iter()
            .filter(|word| {
                let bytes = word.as_bytes();
                let first_letter = char_shift(to_upper(bytes[0]));
                let match_index = if first_letter & line_one != 0 {
                    line_one
                } else if first_letter & line_two != 0 {
                    line_two
                } else {
                    line_three
                };

                bytes.iter().all(|char| char_shift(to_upper(*char)) & match_index != 0)
            })
            .collect::<Vec<String>>()
    }
}

#[test]
fn test_code_500() {
    println!(
        "{:?}",
        Solution::find_words(vec!["qwefx".to_owned(), "Dad".to_owned(), "sfd".to_owned()])
    )
}
