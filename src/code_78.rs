use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Solution::subset_internal(&nums, 0, &mut Vec::new(), &mut result);

        result
    }

    fn subset_internal(
        numbers: &Vec<i32>,
        index: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(current.clone());

        for index in index..numbers.len() {
            let number = numbers[index];

            current.push(number);
            Solution::subset_internal(numbers, index + 1, current, result);
            current.pop();
        }
    }
}

#[test]
fn test_code_78() {
    assert_eq!(
        Solution::subsets(vec![1, 2]),
        vec![vec![], vec![1], vec![1, 2], vec![2]]
    );

    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3]
        ]
    );

    assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
}
