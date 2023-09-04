use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>,
                                  p: Option<Rc<RefCell<TreeNode>>>,
                                  q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
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

#[test]
fn test_code_235() {
    println!("{:?}", Solution::lowest_common_ancestor(tree_node_of![1,2,3,4,5,6,7,8,9],
                                                      tree_node_of![4],
                                                      tree_node_of![3]).unwrap().borrow().val);
}