use std::cmp::max;
use std::rc::Rc;
use crate::Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut remember = Vec::<i32>::new();
        remember.resize(amount as usize + 1, 0);
        remember[0] = 1;

        for i in 0..coins.len() {
            for index in 1..remember.len() {
                let value = coins[i];
                let last_coin_index = index as i32 - value;
                if last_coin_index >= 0 {
                    let last_coin_count = remember[last_coin_index as usize];
                    remember[index] = remember[index] + last_coin_count;
                }
            }
        }

        return *remember.last().unwrap();
    }
}