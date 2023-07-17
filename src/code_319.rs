use crate::Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let mut lighting_count = 1;

        loop {
            let lighting_sum = lighting_count * lighting_count;
            if lighting_sum > n {
              break;  
            } 

            lighting_count += 1;
        }

        return lighting_count - 1;
    }
}

#[test]
fn test_code_319() {
    assert_eq!(Solution::bulb_switch(1), 1);
    assert_eq!(Solution::bulb_switch(4), 2);
    assert_eq!(Solution::bulb_switch(2), 1);
    assert_eq!(Solution::bulb_switch(3), 1);
    assert_eq!(Solution::bulb_switch(9), 3);
}