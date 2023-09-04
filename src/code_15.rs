use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();

        nums.iter().enumerate().for_each(|(index, num)| {
            let mut left_index = index + 1;
            let mut right_index = nums.len() - 1;

            if index > 0 && num == &nums[index - 1] {
                return;
            }

            let remain_target = -num;

            while left_index < right_index {
                if left_index > index + 1 && nums[left_index] == nums[left_index - 1] {
                    left_index += 1;
                    continue;
                }

                if right_index < nums.len() - 1 && nums[right_index] == nums[right_index + 1] {
                    right_index -= 1;
                    continue;
                }

                let cmp_result = (nums[left_index] + nums[right_index]).cmp(&remain_target);
                match cmp_result {
                    std::cmp::Ordering::Less => {
                        left_index += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(vec![*num, nums[left_index], nums[right_index]]);
                        left_index += 1;
                        right_index -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        right_index -= 1;
                    }
                }
            }
        });

        return result;
    }
}

#[test]
fn test_code_15() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!("{:?}", Solution::three_sum(vec![-2,0,0,2,2]));
}