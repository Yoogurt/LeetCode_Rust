use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            node.left = node.right.take();
            node.right = left;

            Solution::invert_tree(node.left.clone());
            Solution::invert_tree(node.right.clone());
        }

        root
    }
}

#[test]
fn test_code_226() {
    println!("{:?}", Solution::invert_tree(tree_node_of![1,2,3,4,5,6,7]));
}