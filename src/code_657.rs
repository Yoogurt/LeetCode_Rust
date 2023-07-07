use crate::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut position = (0, 0);

        moves.chars().for_each(|char| match char {
            'U' => {
                position.1 += 1;
            }
            'D' => {
                position.1 -= 1;
            }
            'A' => {
                position.0 -= 1;
            }
            'D' => {
                position.0 += 1;
            }
            _ => {}
        });

        position == (0, 0)
    }
}

#[test]
fn test_code_657(){ 
    assert_eq!(Solution::judge_circle("".to_owned()), true);
    assert_eq!(Solution::judge_circle("".to_owned()), true);
}