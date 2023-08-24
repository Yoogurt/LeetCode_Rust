use crate::Solution;

impl Solution {
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min_value = i32::MAX;

        prices.iter().for_each(|value| {
            result = std::cmp::max(result, *value - min_value);
            min_value = std::cmp::min(min_value, *value);
        });

        result
    }
}

#[test]
fn test_code_121() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 2]));
    println!("{}", Solution::max_profit(vec![7, 6, 5, 4, 2, 1]));
}
