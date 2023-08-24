use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() || &s[0..1] == "0" {
            return 0;
        }

        let mapping = (1..=26)
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        let is_validate_string = |chars: &str| mapping.contains(&chars.to_owned());

        let mut result = Vec::<i32>::new();
        result.resize(s.len() + 1, 0);
        result[0] = 1;

        for index in 0..s.len() {
            if is_validate_string(&s[index..index + 1]) {
                result[index + 1] += result[index];
            }

            if index > 0 && is_validate_string(&s[index - 1..index + 1]) {
                result[index + 1] += result[index - 1];
            }
        }

        *result.last().unwrap()
    }
}

#[test]
fn test_code_91() {
    // assert_eq!(Solution::num_decodings("12".to_owned()), 2);
    // assert_eq!(Solution::num_decodings("128".to_owned()), 2);
    // assert_eq!(Solution::num_decodings("123".to_owned()), 3);
    // assert_eq!(Solution::num_decodings("06".to_owned()), 0);
    assert_eq!(Solution::num_decodings("106".to_owned()), 1);
}