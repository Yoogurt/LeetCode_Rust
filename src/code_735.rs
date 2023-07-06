use crate::Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        if asteroids.is_empty() {
            return vec![];
        }

        let mut result = Vec::<i32>::new();
        result.push(asteroids[0]);

        for index in 1..asteroids.len() {
            let current = asteroids[index];

            let mut collision_happen = false;

            while let Some(last) = result.last() {
                if *last > 0 && current < 0 {
                    let abs_last = last.abs();
                    let abs_current = current.abs();

                    if abs_last > abs_current {
                        collision_happen = true;
                        break;
                    } else if abs_last == abs_current {
                        result.pop();
                        collision_happen = true;
                        break;
                    } else {
                        result.pop();
                    }
                } else {
                    break;
                }
            }

            if !collision_happen {
                result.push(current);
            }
        }

        result
    }
}

#[test]
fn test_code_735() {
    assert_eq!(Solution::asteroid_collision(vec![5, -10]), vec![-10]);
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), Vec::<i32>::new());
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
}
