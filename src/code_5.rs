use crate::Solution;

impl Solution {
    fn double_palindrome_length_of(data: &Vec<char>, index: i32) -> String {
        let mut left_index = index;
        let mut right_index = index + 1;

        while left_index >= 0 && right_index < data.len() as i32 {
            if data[left_index as usize] == data[right_index as usize] {
                left_index -= 1;
                right_index += 1;
            } else {
                break;
            }
        }

        if left_index == right_index - 1 {
            return "".to_string();
        }

        data[(left_index + 1) as usize ..= (right_index - 1) as usize].iter().collect()
    }

    fn single_palindrome_length_of(data: &Vec<char>, index: i32) -> String {
        let mut left_index = index - 1;
        let mut right_index = index + 1;

        while left_index >= 0 && right_index < data.len() as i32 {
            if data[left_index as usize] == data[right_index as usize] {
                left_index -= 1;
                right_index += 1;
            } else {
                break;
            }
        }

        data[(left_index + 1) as usize ..= (right_index - 1) as usize].iter().collect()
    }

    pub fn longest_palindrome(s: String) -> String {
        let data: Vec<char> = s.chars().collect();
        let mut result: String = String::default();

        for (index, _) in data.iter().enumerate() {
            {
                let single_palindrome_length =  Solution::single_palindrome_length_of(&data, index as i32);
                if single_palindrome_length.len() > result.len() {
                    result = single_palindrome_length;
                }

                let double_palindrome_length =  Solution::double_palindrome_length_of(&data, index as i32);
                if double_palindrome_length.len() > result.len() {
                    result = double_palindrome_length;
                }
            }
        }

        result
    }
}