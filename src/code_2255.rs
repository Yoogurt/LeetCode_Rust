use Base::vec_string_convert::Vec2OwnedString;
use crate::Solution;

impl Solution {
     fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|&w| s.starts_with(w)).count() as i32
    }
}

#[test]
fn test_code_2255() {
    println!("{}", Solution::count_prefixes(["a", "b", "c", "ab", "bc", "abc"].to_owned_string(), "abc".to_string()));
}