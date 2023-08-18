use crate::Solution;

const VISIT: i32 = 1;
const IN: i32 = 2;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut result = vec![0; n as usize];

        edges.iter().all(|value| {
            let flag1_index = value[0] as usize;
            let flag1 = result[flag1_index];

            let flag2_index = value[1] as usize;
            let flag2 = result[flag2_index];

            result[flag1_index] = result[flag1_index] | VISIT;

            result[flag2_index] = result[flag2_index] | VISIT | IN;
            return flag1 & VISIT == 0 || flag2 & VISIT == 0;
        }) && result.iter().all(|value| *value & VISIT != 0)
            && result.iter().filter(|value| *value & IN == 0).count() == 1
    }
}

#[test]
fn test_code_261() {
    // println!(
    //     "{}",
    //     Solution::valid_tree(
    //         5,
    //         vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
    //     )
    // );

    // println!(
    //     "{}",
    //     Solution::valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]])
    // );

    println!(
        "{}",
        Solution::valid_tree(4, vec![vec![0, 1], vec![2, 3], vec![1, 3]])
    );
}
