
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

        match root.clone() {
            None => { return None; }
            Some(tree_node) => {
                let left = Solution::lowest_common_ancestor(tree_node.borrow().left.clone(), p.clone(), q.clone());
                let right = Solution::lowest_common_ancestor(tree_node.borrow().right.clone(), p.clone(), q.clone());

                if left.is_some() && right.is_some() {
                    return root;
                }

                let val = tree_node.borrow().val;
                if let Some(p) = p {
                    if p.borrow().val == val {
                        return root;
                    }
                }

                if let Some(q) = q {
                    if q.borrow().val == val {
                        return root;
                    }
                }

                if left.is_some() {
                    return left;
                }

                if right.is_some() {
                    return right;
                }

                None
            }
        }
    }
}