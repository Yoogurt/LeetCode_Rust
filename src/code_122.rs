use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut last_value = i32::MAX;

        prices.into_iter().for_each(|value| {
            let profit = value - last_value;
            if profit > 0 {
                result += profit;
            }

            last_value = value;
        });

        result
    }
}

#[test]
fn test_code_122() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 2]))
}
