use crate::Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut max_flowers = 0;

        for index in 0..flowerbed.len() {
            if flowerbed[index] == 1 {
                continue;
            }

            let left_flower = if index > 0 {
                *flowerbed.get(index - 1).unwrap_or(&0)
            } else {
                0
            };
            let right_flower = *flowerbed.get(index + 1).unwrap_or(&0);

            if left_flower == 0 && right_flower == 0 {
                flowerbed[index] = 1;
                max_flowers += 1;
            }
        }

        return max_flowers >= n;
    }
}

#[test]
fn test_code_605() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}
