use Base::vec_string_convert::Vec2OwnedString;
use crate::Solution;

impl Solution {
    pub fn find_string(words: Vec<String>, s: String) -> i32 {
        let words = words.iter().enumerate().filter_map(|(index, value)| {
            if value.is_empty() {
                None
            } else {
                Some((index, value))
            }
        }).collect::<Vec<_>>();

        let result = words.binary_search_by(|tunple| {
            tunple.1.cmp(&s)
        });

        result.map_or(-1, |index| {
            words[index].0 as i32
        })
    }
}

#[test]
fn test_code_1005() {
    println!("{}", Solution::find_string(vec!["at", "", "", "", "ball", "", "", "car", "", "","dad", "", ""].to_owned_string(), "ball".into()))
}