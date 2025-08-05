use crate::Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        if fruits.is_empty() {
            return 0;
        }

        let mut result = 0;
        let mut last_kind_record = ((-1, 0), (-1, 0));
        let mut left_index = 0;

        fruits.iter().for_each(|&kind| {
            if last_kind_record.0 .0 == kind {
                last_kind_record.0 .1 += 1;
                result = result.max(last_kind_record.0 .1 + last_kind_record.1 .1);
            } else if last_kind_record.1 .0 == kind {
                last_kind_record.1 .1 += 1;
                result = result.max(last_kind_record.0 .1 + last_kind_record.1 .1);
            } else {
                if last_kind_record.0 .0 == -1 {
                    last_kind_record.0 .0 = kind;
                    last_kind_record.0 .1 = 1;
                } else if last_kind_record.1 .0 == -1 {
                    last_kind_record.1 .0 = kind;
                    last_kind_record.1 .1 = 1;
                } else {
                    loop {
                        if fruits[left_index] == last_kind_record.0 .0 {
                            left_index += 1;
                            last_kind_record.0 .1 -= 1;
                            if last_kind_record.0 .1 == 0 {
                                last_kind_record.0.0 = kind;
                                last_kind_record.0.1 = 1;
                                break;
                            }
                        } else {
                             left_index += 1;
                            last_kind_record.1 .1 -= 1;
                            if last_kind_record.1 .1 == 0 {
                                last_kind_record.1.0 = kind;
                                last_kind_record.1.1 = 1;
                                break;
                            }
                        }
                    }
                }

                result = result.max(last_kind_record.0 .1 + last_kind_record.1 .1);
            }
        });

        result
    }
}

#[test]
fn test_code_904() {
    dbg!(Solution::total_fruit(vec![0,1,2,0,0,0]));
}
