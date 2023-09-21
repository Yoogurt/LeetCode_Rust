use crate::Solution;

impl Solution {
    pub fn transform_array(mut arr: Vec<i32>) -> Vec<i32> {
        let mut next_array = vec![0; arr.len()];
        let len = arr.len();

        let mut first_array = &mut arr;
        let mut second_array = &mut next_array;

        let mut changed = true;
        while changed {
            changed = false;

            first_array.iter().enumerate().for_each(|(index,  &value)| {
                if index == 0 || index == len - 1 {
                    second_array[index] = value;
                    return
                }

                let left_compare_result = value.cmp(&first_array[index - 1]);
                let right_compare_result = value.cmp(&first_array[index + 1]);

                if left_compare_result.is_gt() && right_compare_result.is_gt() {
                    second_array[index] = value - 1;
                    changed = true;
                } else if left_compare_result.is_lt() && right_compare_result.is_lt() {
                    second_array[index] = value + 1;
                    changed = true;
                } else {
                    second_array[index] = value;
                }
            });

            let temp = first_array;
            first_array = second_array;
            second_array = temp;
        }

        arr
    }
}

#[test]
fn test_code_1243() {
    dbg!(Solution::transform_array(vec![1,6,3,4,3,5]));
}