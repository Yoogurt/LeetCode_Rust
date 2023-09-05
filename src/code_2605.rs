use std::borrow::Borrow;
use crate::Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.is_empty() {
            return *nums2.iter().min().unwrap_or(&0);
        }

        if nums2.is_empty() {
            return *nums1.iter().min().unwrap_or(&0);
        }

        let mut min = nums1.first().unwrap();
        let mut bit = 1 << nums1.first().unwrap();

        bit = nums1.iter().skip(1).fold(bit, |acc, value| {
            min = min.min(value);
            acc | (1 << value)
        });

        let mut min_2 = i32::MAX;

        let mut found: Option<i32> = None;
        for index in 0..nums2.len() {
            let value = nums2[index];
            if bit & (1 << value) != 0 {
                if let Some(found_value) = found {
                    if found_value > value {
                        found = Some(value);
                    }
                } else {
                    found = Some(value);
                }
            }

            min_2 = min_2.min(value);
        }

        if let Some(found_value) = found {
            return found_value;
        }

        min.min(&min_2) * 10 + min.max(&min_2)
    }
}

#[test]
fn test_code_2605() {
    println!("{}", Solution::min_number(vec![1, 4, 5, 6], vec![2, 4, 3, 1]))
}