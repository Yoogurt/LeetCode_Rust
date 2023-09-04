use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = 0;
        let mut right_max = vec![0; height.len()];

        // calc right max slider
        height
            .iter()
            .rev()
            .enumerate()
            .fold(0, |max, (index, current)| {
                // visit begin from right side
                let index = right_max.len() - index - 1;
                right_max[index] = max;
                std::cmp::max(max, *current)
            });

        height
            .into_iter()
            .enumerate()
            .fold(0, |trapped, (index, current)| {
                let left_right_min = std::cmp::min(left_max, right_max[index]);
                left_max = std::cmp::max(left_max, current);
                // can trap rain ?
                trapped + std::cmp::max(left_right_min - current, 0)
            })
    }
}

#[test]
fn test_code_42() {
    println!(
        "{}",
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    );
}
