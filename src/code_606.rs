use std::{cell::RefCell, rc::Rc};

use Base::{tree_node::TreeNode, tree_node_of};

use crate::Solution;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_owned();
        }

        let root = root.unwrap();

        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        let left_result = Solution::tree2str(left);
        let right_result = Solution::tree2str(right);

        let mut result = root.borrow().val.to_string();

        if left_result.is_empty() && right_result.is_empty() {
            return result;
        }

        result = std::format!("{result}({left_result})");

        if !right_result.is_empty() {
            result = std::format!("{result}({right_result})");
        }

        return result;
    }
}

#[test]
fn test_code_606() {
    assert_eq!(Solution::tree2str(tree_node_of![1, 2, 3, -1, 4]), "1(2()(4))(3)");
    assert_eq!(Solution::tree2str(tree_node_of![1, 2, 3, 4]), "1(2(4))(3)");
}
