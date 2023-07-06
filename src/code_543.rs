use std::{cell::RefCell, rc::Rc};

use Base::{
    tree_node::*,
    tree_node_of,
};

use crate::Solution;

impl Solution {
    fn depth_of(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();

        let left_depth = Solution::depth_of(root.borrow().left.clone(), result);
        let right_depth = Solution::depth_of(root.borrow().right.clone(), result);

        *result = std::cmp::max(*result, left_depth + right_depth);
        return std::cmp::max(left_depth, right_depth) + 1;
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut result = 0;
        Solution::depth_of(root, &mut result);
        result
    }
}

#[test]
fn test_code_543() {
    assert_eq!(
        Solution::diameter_of_binary_tree(tree_node_of![1, 2, 3, 4, 5]),
        3
    );
    assert_eq!(Solution::diameter_of_binary_tree(tree_node_of![1, 2]), 1);
}
