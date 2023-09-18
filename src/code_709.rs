use crate::Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        s.iter_mut().for_each(|value| {
            if *value >= 'A' && *value <= 'Z' {
                *value = char::from_u32(*value as u32 - 'A' as u32 + 'a' as u32).unwrap();
            }
        });
        String::from_iter(s)
    }
}

#[test]
fn test_code_709() {
    dbg!(Solution::to_lower_case("ASDC".to_owned()));
}