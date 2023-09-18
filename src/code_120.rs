use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }

        let mut result = vec![0; triangle.last().unwrap().len()];
        let mut result_back = vec![0; triangle.last().unwrap().len()];

        triangle.into_iter().for_each(|layer| {
            if layer.is_empty() {
                return;
            }

            let final_index = layer.len() - 1;
            std::mem::swap(&mut result, &mut result_back);

            layer.iter().enumerate().for_each(|(index, weight)| {
                if index == 0 {
                    result[index] = result_back[index] + weight;
                } else if index == final_index {
                    result[index] = result_back[index - 1] + weight;
                } else {
                    result[index] = result_back[index].min(result_back[index - 1]) + weight;
                }
            });
        });

        result.into_iter().min().unwrap_or(0)
    }
}

#[test]
fn test_code_120() {
    let data = vec![vec![]];
    dbg!(Solution::minimum_total(data));
}