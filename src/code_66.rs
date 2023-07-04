use crate::Solution;
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let last_over_10 = Rc::new(RefCell::new(false));

        let mut result = digits
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, value)| {
                let mut result = if index == 0 { value + 1 } else { value };
                let over_10_ref = last_over_10.clone();
                {
                    let over_10 = over_10_ref.borrow();
                    result += if *over_10 { 1 } else { 0 };
                }

                over_10_ref.replace(result >= 10);

                result % 10
            })
            .collect::<Vec<i32>>();

        result.reverse();


        if *last_over_10.as_ref().borrow() {
            result.insert(0, 1);
        }

        result
    }
}

#[test]
fn test_solution_66() {
    assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    assert_eq!(vec![1, 3, 0], Solution::plus_one(vec![1, 2, 9]));
    assert_eq!(vec![1, 0, 0, 0], Solution::plus_one(vec![9, 9, 9]));
    assert_eq!(vec![1], Solution::plus_one(vec![0]));
}
