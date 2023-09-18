use crate::Solution;

impl Solution {
    fn validate_palindrome(s: &[char],
                           left_index: &mut usize,
                           right_index: &mut usize,
                           mut allow_deletion: bool) -> bool {
        while left_index < right_index {
            if s[*left_index] != s[*right_index] {
                if !allow_deletion {
                    return false;
                }

                allow_deletion = false;

                let left =s[*left_index];
                let right = s[*right_index];
                let left_left = s[*left_index + 1];
                let right_right = s[*right_index - 1];

                let left_result = if s[*left_index + 1] == s[*right_index] {
                    let mut left_index = *left_index + 1;
                    let mut right_index = *right_index;
                    Solution::validate_palindrome(s, &mut left_index, &mut right_index, false)
                } else {
                    false
                };
                let right_result = if s[*left_index] == s[*right_index - 1] {
                    let mut left_index = *left_index;
                    let mut right_index = *right_index - 1;
                    Solution::validate_palindrome(s, &mut left_index, &mut right_index, false)
                } else {
                    false
                };

                return left_result || right_result
            }

            *left_index += 1;
            *right_index -= 1;
        }

        true
    }
    pub fn valid_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut s = s.chars().collect::<Vec<_>>();

        let mut left_index = 0;
        let mut right_index = s.len() - 1;

        Solution::validate_palindrome(&s, &mut left_index, &mut right_index, true)
    }
}

#[test]
fn test_code_680() {
    dbg!(Solution::valid_palindrome("yd".to_owned()));
}