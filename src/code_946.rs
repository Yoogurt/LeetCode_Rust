use crate::Solution;

impl Solution {
    pub fn validate_stack_sequences(mut pushed: Vec<i32>, mut popped: Vec<i32>) -> bool {
        let mut stack = Vec::<i32>::new();

        while !popped.is_empty() {
            if stack.is_empty() {
                if pushed.is_empty() {
                    return false;
                }

                stack.push(pushed.remove(0));
                continue;
            }

            if stack.last() == popped.first() {
                stack.remove(stack.len() - 1);
                popped.remove(0);
                continue;
            }

            if pushed.is_empty() {
                return false;
            }
            stack.push(pushed.remove(0));
        }

        popped.is_empty() && pushed.is_empty()
    }
}

#[test]
fn test_code_946() {
    // assert_eq!(
    //     Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
    //     true
    // );

    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
        false
    );
}
