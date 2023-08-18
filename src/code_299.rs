use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut hash_table = HashMap::<char, i32>::new();
        secret.chars().for_each(|char| {
            hash_table.insert(char, hash_table.get(&char).unwrap_or(&0) + 1);
        });

        let mut cow = 0;
        guess.chars().for_each(|char| {
            let contain_key = hash_table.get_mut(&char);

            match contain_key {
                Some(count) => {
                    if count > &mut 0 {
                        cow += 1;
                        *count -= 1;
                    }
                }
                _ => {}
            }
        });

        let mut bull = 0;
        secret
            .chars()
            .zip(guess.chars())
            .for_each(|(left_char, right_char)| {
                if left_char == right_char {
                    bull += 1;
                }
            });

        return std::format!("{}A{}B", bull, std::cmp::max(0, cow - bull));
    }
}

#[test]
fn test_code_299() {
    println!("{}", Solution::get_hint("1807".into(), "7810".into()));
    println!("{}", Solution::get_hint("1123".into(), "0111".into()))
}
