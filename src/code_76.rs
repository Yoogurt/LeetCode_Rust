use crate::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut chars = [0usize; 'z' as usize - 'A' as usize];
        let mut result = s.clone();
        let index = 'A' as usize;
        let mut bit_remember = 0u64;
        t.chars().for_each(|value| {
            let value_char = value as usize;
            chars[value_char - index] += 1;
            let shift = 1u64 << (value_char - index);
            bit_remember |= shift;
        });

        let s = s.as_bytes();

        let mut total = t.len();

        let mut start_index = 0usize;
        let mut end_index = 0usize;

        loop {
            while total > 0 && end_index < s.len() {
                let char_in_target = s[end_index];

                if chars[char_in_target as usize - index] > 0 {
                    chars[char_in_target as usize - index] -= 1;
                    total -= 1;
                }

                end_index += 1;
            }

            while start_index < end_index {
                let value_char = s[start_index] as usize - index;

                if bit_remember & (1 << value_char) != 0 {
                    if chars[value_char] > 0 {
                        chars[value_char] -= 1;
                        total -= 1;
                    } else {
                        break;
                    }
                }
                start_index += 1;
            }

            if total == 0 && end_index - start_index < result.len() {
                result = String::from_utf8_lossy(&s[start_index..end_index]).to_string();
            }

            if start_index < end_index {
                let value_char = s[start_index] as usize - index;

                if bit_remember & (1 << value_char) == 0 {
                    unreachable!();
                } else {
                    total += 1;
                    chars[value_char] += 1;
                    start_index += 1;

                    while start_index < end_index {
                        let value_char = s[start_index] as usize - index;

                        if bit_remember & (1 << value_char) != 0 {
                            break;
                        } else {
                            start_index += 1;
                        }
                    }
                }
            } else {
                break;
            }
        }

        result
    }
}

#[test]
fn test_code_76() {
    // assert_eq!(Solution::min_window("qweqfas".to_owned(), "wef".to_owned()), "weqf".to_owned());
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
        "BANC".to_owned()
    );
    // assert_eq!(Solution::min_window("a".to_owned(), "a".to_owned()), "a".to_owned());
    // assert_eq!(Solution::min_window("a".to_owned(), "aa".to_owned()), "".to_owned());
}
