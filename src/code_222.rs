use std::rc::Rc;
use std::cell::RefCell;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn visit_node(node: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) {
            match node {
                None => {}
                Some(node) => {
                    *result += 1;
                    visit_node(node.borrow().left.clone(), result);
                    visit_node(node.borrow().right.clone(), result);
                }
            }
        }

        let mut result = 0;
        visit_node(root, &mut result);
        result
    }
}