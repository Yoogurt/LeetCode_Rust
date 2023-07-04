use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let data: Vec<char> = s.chars().collect();

        let mut stack = Vec::<char>::new();

        for it in data {
            match it {
                '('|'['|'{' => {
                    stack.push(it);
                }
                ')' => {
                    match stack.pop() {
                        Some('(') => {

                        }
                        _=> { return false; }
                    }
                }
                ']' => {
                    match stack.pop() {
                        Some('[') => {

                        }
                        _=> { return false; }
                    }
                }
                '}' => {
                    match stack.pop() {
                        Some('{') => {}
                        _=> { return false; }
                    }
                }
                _ => {
                    panic!("unexpected char")
                }
            }
        }

        return stack.is_empty();
    }
}