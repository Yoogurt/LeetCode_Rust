use crate::Solution;

impl Solution {
    pub fn remove_vowels(s: String) -> String {
        String::from_iter(s.chars().filter(|value| {
            match value {
                'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => {
                    false
                }
                _ => {
                    true
                }
            }
        }))
    }
}