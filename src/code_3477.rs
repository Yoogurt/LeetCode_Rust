use crate::Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(mut fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut result = 0;
        fruits.into_iter().for_each(|fruit| {
            for basket in &mut baskets {
                if fruit <= *basket {
                    *basket = -1;
                    return;
                }
            }

            result += 1;
        });
        result
    }
}

#[test]
fn test_code_3477() {
    dbg!(Solution::num_of_unplaced_fruits(
        vec![4, 2, 5],
        vec![3, 5, 4]
    ));
}
