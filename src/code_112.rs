use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => {
                return target_sum == 0;
            }
            Some(tree_node) => {
                let tree_node = tree_node.borrow();
                let target_sum = target_sum - tree_node.val;
                Solution::has_path_sum(tree_node.left.clone(), target_sum) ||
                    Solution::has_path_sum(tree_node.right.clone(), target_sum)
            }
        }
    }
}