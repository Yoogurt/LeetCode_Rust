use crate::Solution;

impl Solution {
    fn permute_unique_internal(remain: &mut Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remain.is_empty() {
            result.push(current.clone());
            return;
        }

        let length = remain.len();
        let mut index = 0;
        let mut last_pick: Option<i32> = None;
        while index < length {
            let current_pick = remain[index];
            if let Some(last_pick) = last_pick {
                if last_pick == current_pick {
                    index += 1;
                    continue
                }
            }

            remain.remove(index);
            current.push(current_pick);
            Solution::permute_unique_internal(remain, current, result);
            current.pop();
            remain.insert(index, current_pick);
            last_pick = Some(current_pick);
            index += 1;
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        nums.sort();

        Solution::permute_unique_internal(&mut nums, &mut vec![], &mut result);

        return result;
    }
}