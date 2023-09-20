use crate::Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut counts = [0i32; 10];

        num.chars().for_each(|value| {
            let index = value as usize - '0' as usize;
            if counts[index] == -1 {
                counts[index] = 1;
            } else {
                counts[index] += 1;
            }
        });


        num.chars().enumerate().all(|(index, value)| {
            let char_index = value as i32 - '0' as i32;
            counts[index] >= char_index
        })
    }
}

#[test]
fn test_code_2283() {
    println!("{}", Solution::digit_count("030".to_owned()));
}