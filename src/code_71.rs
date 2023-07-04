use crate::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        let segment = path.split("/");
        for segment_path in segment {
            match segment_path {
                ""|"." => {}
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(segment_path);
                }
            }
        }

       
        let result = stack.join("/");
        std::format!("/{result}")
    } 
}

#[test]
fn test_code_71() {
    assert_eq!("/home", Solution::simplify_path("/home/".to_owned()));
    assert_eq!("/", Solution::simplify_path("/home/..".to_owned()));
    assert_eq!("/", Solution::simplify_path("/../..".to_owned()));
    assert_eq!(
        "/home/mariklin",
        Solution::simplify_path("/home/../home/mariklin".to_owned())
    );
}
