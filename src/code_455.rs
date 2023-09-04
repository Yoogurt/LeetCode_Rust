use crate::Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut satify = 0;
        let mut visit_index = 0;
        loop {
            if satify >= s.len() || visit_index >= g.len() {
                break;
            }

            let needed = g[visit_index];

            if needed <= s[satify] {
                visit_index += 1;
            }

            satify += 1;
        }

        satify.min(visit_index) as i32
    }
}

#[test]
fn test_code_455() {
    println!("{}", Solution::find_content_children(vec![1, 2, 3], vec![1, 1]));
    println!("{}", Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]));
}