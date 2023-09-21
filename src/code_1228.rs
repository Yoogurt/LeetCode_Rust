use crate::Solution;

impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let mut min = arr[0];
        let mut max = arr[0];
        let mut sum = arr[0];
        let mut len = (arr.len() + 1) as i32;
        arr.into_iter().skip(1).for_each(|value| {
            min = min.min(value);
            max = max.max(value);
            sum += value;
        });

        (len * (min + max) >> 1) - sum
    }
}

#[test]
fn test_code_1227() {
    dbg!(Solution::missing_number(vec![1,3,4]));
}