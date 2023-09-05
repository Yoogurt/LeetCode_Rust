use crate::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();

        let mut left_index = 0;
        let mut right_index = s.len() - 1;
        let vowels = "aeiou".chars().fold(0, |acc, value| {
            acc | (1 << (value as i32 - 'a' as i32))
        }); 

        let _a = vowels & (1 << (s[left_index] as i32 - 'a' as i32)) == 0;

        while left_index < right_index {
            if vowels & (1 << (s[left_index] as i32 - 'a' as i32)) == 0 {
                left_index += 1;
                continue;
            }

            if vowels & (1 << (s[right_index] as i32 - 'a' as i32)) == 0 {
                right_index -= 1;
                continue;
            }

            let temp = s[right_index];
            s[right_index] = s[left_index];
            s[left_index] = temp;

            left_index += 1;
            right_index -= 1;
        }

        return String::from_iter(s);
    }
}

#[test]
fn test_code_345() {
    println!("{}", Solution::reverse_vowels("waejknxzioli".to_owned()))
}