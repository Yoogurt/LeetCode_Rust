use crate::Solution;

impl Solution {
    pub fn is_armstrong(n: i32) -> bool {
        let mut bits = vec![0; 0];

        let mut remain = n;
        while remain > 0 {
            bits.push(remain % 10);
            remain /= 10;
        }

        bits.iter().fold(0, |acc, value| {
            acc + value.pow(bits.len() as u32)
        }) == n
    }
}

#[test]
fn test_code_1134() { dbg!(Solution::is_armstrong(0)); }