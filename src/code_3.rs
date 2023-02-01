use std::cmp::max;
use std::collections::HashSet;
use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let s : Vec<char> = s.chars().collect();

        let mut set = HashSet::<char>::new();

        let mut start_index: usize = 0;
        let mut scan_index:usize = 0;
        let mut longest: usize = 0;

        while scan_index < s.len() {

            if !set.contains(&s[scan_index]) {
                set.insert(s[scan_index]);
                scan_index += 1;
                longest = max(longest, scan_index - start_index)
            } else {
                set.remove(&s[start_index]);
                start_index += 1;
            }
        }

        return longest as i32;
    }
}