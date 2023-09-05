use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;

use crate::Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => { 0 }
            Some(node) => {
                match (node.borrow().left.clone(), node.borrow().right.clone()) {
                    (Some(left), Some(right)) => {
                        1 + Solution::min_depth(Some(left)).min(Solution::min_depth(Some(right)))
                    }
                    (Some(left), None) => {
                        Solution::min_depth(Some(left)) + 1
                    }
                    (None, Some(right)) => {
                        Solution::min_depth(Some(right)) + 1
                    }
                    _ => { 1 }
                }
            }
        }
    }
}

#[test]
fn test_code_111() {
    println!("{}", Solution::min_depth(tree_node_of![2, -1, 3, -1, 4, -1, 5, -1 ,6]));
    println!("{}", Solution::min_depth(tree_node_of![3,9,20,-1,-1,15,7]));
}