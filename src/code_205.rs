
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut hash_count = [0u32; 26];
        s.as_bytes().iter().for_each(|value| {
            hash_count[(*value - 'a' as u8) as usize] += 1;
        });
        let mut hash_mapping = HashMap::<u32, i32>::new();
        hash_count.iter().for_each(|value| {
            let count = hash_mapping.get(value).unwrap_or(&0);
            hash_mapping.insert(*value, count + 1);
        });

        let mut target_hash_count = [0u32; 26];
        t.as_bytes().iter().for_each(|value| {
            target_hash_count[(*value - 'a' as u8) as usize] += 1;
        });

        if target_hash_count == hash_count {
            return false;
        }

        target_hash_count.iter().for_each(|value| {
            let count = hash_mapping.get(value).unwrap_or(&0);
            hash_mapping.insert(*value, count - 1);
        });

        hash_mapping.into_iter().all(|(_, value)| value == 0)
    }
}

#[test]
fn test_code_205() {
    println!("{}", Solution::is_isomorphic("paper".into(), "title".into()));
    println!("{}", Solution::is_isomorphic("egg".into(), "obb".into()));
    println!("{}", Solution::is_isomorphic("egg".into(), "bar".into()));
    println!("{}", Solution::is_isomorphic("egg".into(), "gge".into()));
}
