use crate::Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut result = vec![-1; (amount + 1) as usize];
        result[0] = 0;

        coins.sort();

        for index in 1..result.len() {
            coins.iter().any(|coin| {
                if (*coin as usize) > index {
                    return true;
                }
                if *coin == index as i32 {
                    result[index] = 1;
                    return false;
                }
                let previous_coin_index = index - *coin as usize;
                if result[index] == -1 {
                    if result[previous_coin_index] != -1 {
                        result[index] = result[previous_coin_index] + 1;
                    }
                } else {
                    if result[previous_coin_index] != -1 {
                        result[index] =
                            std::cmp::min(result[index], result[previous_coin_index] + 1);
                    }
                }
                false
            });
        }

        *result.last().unwrap()
    }
}

#[test]
fn test_code_322() {
    // println!("{}", Solution::coin_change(vec![1, 2, 5], 13));
    // println!("{}", Solution::coin_change(vec![2], 3));
    // println!("{}", Solution::coin_change(vec![1], 0));
    println!("{}", Solution::coin_change(vec![186, 419, 83, 408], 6249))
}
