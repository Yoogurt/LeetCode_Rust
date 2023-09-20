use std::iter;
use crate::Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if a.len() > b.len() {
            let c = a;
            a = b;
            b = c;
        }

        let a = a.chars().rev();
        let b = b.chars().rev();
        let mut ret: Vec<char> = vec![];

        let acc = b.zip(a.chain(iter::repeat('0'))).fold(0, |mut acc, (left, right)| {
            let result = acc + (left as i32 - '0' as i32) + (right as i32 - '0' as i32);
            ret.push(match result & 1 {
                0 => {
                    '0'
                }
                1 => {
                    '1'
                }
                _ => {
                    '?'
                }
            });

            if result > 1 {
                1
            } else {
                0
            }
        });

        if acc > 0 {
            ret.push('1');
        }

        String::from_iter(ret.iter().rev())
    }
}

#[test]
fn test_code_67() {
    dbg!(Solution::add_binary("0".to_owned(), "100".to_owned()));
}