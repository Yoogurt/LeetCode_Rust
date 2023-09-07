use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.chars();

        !s.chars().any(|value| {
            while let Some(compare) = t.next() {
                if compare == value {
                    return false;
                }
            }

            true
        })
    }
}

#[test]
fn test_code_392() {
    println!("{}", Solution::is_subsequence("abcde".to_owned(), "ahbfc".to_owned()))
}