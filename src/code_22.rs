use crate::Solution;

impl Solution {
    fn generate_parenthesis_rec(left: i32, remain: i32, current: String, result: &mut Vec<String>) {
        if remain <= 0 {
            if left <= 0 {
                result.push(current);
                return;
            }

            Solution::generate_parenthesis_rec(left - 1, remain, current + ")", result);
            return;
        }

        Solution::generate_parenthesis_rec(left + 1, remain - 1, current.clone() + "(", result);

        if left > 0 {
            Solution::generate_parenthesis_rec(left - 1, remain, current + ")", result)
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        Solution::generate_parenthesis_rec(0, n, "".to_string(), &mut result);
        return result;
    }
}