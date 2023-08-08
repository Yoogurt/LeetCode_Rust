use std::ops::Add;

use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();

        tokens.iter().for_each(|value: &String| {
            if value.len() == 1 {
                match &value[0..1] {
                    "+" => {
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(left + right);
                    }
                    "-" => {
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(left - right);
                    }
                    "*" => {
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(left * right);
                    }
                    "/" => {
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(left / right);
                    }
                    _ => {
                        stack.push(value.parse().unwrap());
                    }
                }
            } else {
                stack.push(value.parse().unwrap());
            }
        });

        *stack.last().unwrap()
    }
}

#[test]
fn test_code_150() {
    println!(
        "{}",
        Solution::eval_rpn(
            vec!["2", "1", "+", "3", "*"]
                .iter()
                .map(|value| value.to_string())
                .collect()
        )
    );
    println!(
        "{}",
        Solution::eval_rpn(
            vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                .iter()
                .map(|value| value.to_string())
                .collect()
        )
    );
}
