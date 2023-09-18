use crate::Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut result = 0;

        let mut start_index = 0;
        let mut scan_index = start_index;

        while start_index < s.len() {
            let mut count = 1;
            let char = s[start_index];
            let mut begin_diff_char = false;
            scan_index = start_index + 1;

            while scan_index < s.len() {
                if s[scan_index] == char {
                    if begin_diff_char {
                        break
                    } else {
                        count += 1;
                    }
                } else {
                    if !begin_diff_char {
                        begin_diff_char = true;
                    }
                    count -= 1;
                    result += 1;
                }

                if count == 0 {
                    break
                }

                scan_index += 1;
            }

            while start_index < s.len() && s[start_index] == char  {
                start_index += 1;
            }
        }


        result
    }
}

#[test]
fn test_code_696() {
    dbg!(Solution::count_binary_substrings("0011".to_owned()));
}