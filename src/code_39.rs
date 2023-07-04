use crate::Solution;

impl Solution {
    fn combination(candidates: &Vec<i32>, index: usize, current: &mut Vec<i32>, sum: i32, target: i32, result: &mut Vec<Vec<i32>>) {
        if sum == target {
            result.push(current.clone());
            return;
        } else if sum > target {
            return;
        }
        let mut scan_index = index;
        while scan_index < candidates.len() {
            let it = candidates[scan_index];
            current.push(it);
            Solution::combination(candidates, scan_index, current, sum + it, target, result);
            current.pop();
            scan_index += 1;
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        Solution::combination(&candidates, 0, &mut Vec::<i32>::new(), 0, target, &mut result);
        return result;
    }
}