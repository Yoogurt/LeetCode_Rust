use crate::Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let num = num.chars().collect::<Vec<char>>();

        if num.len() & 1 == 0 {
            let mut start_index = 0;
            let mut end_index = num.len() - 1;

            while start_index < end_index {
                match (num[start_index], num[end_index]) {
                    ('0', '0')|('1', '1') | ('6', '9') | ('9', '6') | ('8', '8') => {}
                    _ => { return false; }
                }

                start_index += 1;
                end_index -= 1;
            }

            true
        } else {
            let mut start_index = 0;
            let mut end_index = num.len() - 1;

            while start_index < end_index {
                match (num[start_index], num[end_index]) {
                    ('0', '0')|('1', '1') | ('6', '9') | ('9', '6') | ('8', '8') => {}
                    _ => { return false; }
                }

                start_index += 1;
                end_index -= 1;
            }

            num[start_index] == '0' || num[start_index] == '1' || num[start_index] == '8'
        }
    }
}

#[test]
fn test_code_246() {
    println!("{}", Solution::is_strobogrammatic("69".into()));
    println!("{}", Solution::is_strobogrammatic("88".into()));
    println!("{}", Solution::is_strobogrammatic("962".into()));
    println!("{}", Solution::is_strobogrammatic("1".into()))
}