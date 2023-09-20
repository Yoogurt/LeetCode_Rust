use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        if gem.is_empty() {
            return 0;
        }

        operations.into_iter().for_each(|operation| {
            let from = operation[0] as usize;
            let to = operation[1] as usize;
            let mut move_candy = gem[from] / 2;

            gem[from] -= move_candy;
            gem[to] += move_candy;
        });

        let mut min = gem[0];
        let mut max = gem[0];

        gem.into_iter().skip(1).for_each(|count| {
            min = min.min(count);
            max = max.max(count);
        });

        max - min
    }
}

#[test]
fn test_code_lcp_50() {
    dbg!(Solution::give_gem(vec![100,0,50,100], [[0,2],[0,1],[3,0],[3,0]].into_vec()));
}