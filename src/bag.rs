use crate::Solution;

impl Solution {
    pub fn wei_bag_problem1(weight: Vec<usize>, value: Vec<usize>, bag_size: usize) -> usize {
        let mut result = Vec::<usize>::new();
        result.resize_with(bag_size + 1, || 0usize);

        for index in 1..result.len() {
            weight
                .iter()
                .enumerate()
                .for_each(|(weight_index, weight)| {
                    if index >= *weight {
                        result[index] = std::cmp::max(
                            result[index - weight] + value[weight_index],
                            result[index],
                        );
                    }
                });
        }

        return *result.last().unwrap() as usize;
    }
}

#[test]
fn test_wei_bag_problem1() {
    println!(
        "{}",
        Solution::wei_bag_problem1(vec![4, 3, 1], vec![30, 20, 15], 4)
    );
}
