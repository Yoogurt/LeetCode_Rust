use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 2 {
            return true;
        }

        let mut first_point = (coordinates[0][0], coordinates[0][1]);
        let mut second_point = (coordinates[1][0], coordinates[1][1]);

        if first_point.0 == second_point.0 {
            return coordinates.into_iter().skip(2).all(|value| {
                value[0] == first_point.0
            });
        }

        if first_point.1 == second_point.1 {
            return coordinates.into_iter().skip(2).all(|value| {
                value[1] == first_point.1
            });
        }

        let a = (second_point.1 - first_point.1) as f32 / (second_point.0 - first_point.0) as f32;
        let b = first_point.1 as f32 - a * first_point.0 as f32;

        coordinates.into_iter().skip(2).all(|value| {
            a * value[0] as f32 + b as f32 == value[1] as f32
        })
    }
}

#[test]
fn test_code_1232() {
    dbg!(Solution::check_straight_line([[2, 1], [4, 2], [6, 3]].into_vec()));
}