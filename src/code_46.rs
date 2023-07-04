use crate::Solution;

impl Solution {
    fn permute_internal(remain: &mut Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remain.is_empty() {
            result.push(current.clone());
            return;
        }

        let mut index = 0;
        let len = remain.len();
        while index < len {
            let used = remain.remove(index);
            current.push(used);
            Solution::permute_internal(remain, current, result);
            current.pop();
            remain.insert(index, used);

            index += 1;
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut nums = nums;
        Solution::permute_internal(&mut nums, &mut Vec::<i32>::new(), &mut result);
        return result;
    }
}