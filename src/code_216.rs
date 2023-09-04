use crate::Solution;

impl Solution {
    fn combination_sum3_internal(k: i32, lower: i32, sum: i32, target: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 1 {
            if target - sum >= lower && target - sum <= 9 {
                let mut ret = current.clone();
                ret.push(target - sum);
                result.push(ret);
            }

            return;
        }

        for lower in lower..=8 {
            current.push(lower);
            Solution::combination_sum3_internal(k - 1, lower + 1, sum + lower, target, current, result);
            current.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Solution::combination_sum3_internal(k, 1, 0, n, &mut vec![], &mut result);
        result
    }
}

#[test]
fn test_code_216() {
    println!("{:?}", Solution::combination_sum3(2, 18))
}