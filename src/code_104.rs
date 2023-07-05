use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut rc = root.unwrap();
        let rc = rc.borrow();
        return max(Solution::max_depth(rc.left.clone()), Solution::max_depth(rc.right.clone())) + 1;
    }
}