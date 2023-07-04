use Base::tree_node::TreeNode;
use Base::tree_node_of;

use crate::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.clone().xor(q.clone()).is_some() {
            return false;
        }
        if p.is_none() {
            return true;
        }

        let p = p.clone().unwrap();
        let q = q.clone().unwrap();

        let p_val = p.borrow().val.clone();
        let q_val = q.borrow().val.clone();

        if p_val != q_val {
            return false;
        }

        return Solution::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
            && Solution::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
    }
}

#[test]
fn test_code_100() {
    assert_eq!(
        Solution::is_same_tree(
            tree_node_of![1, 2, 3, 4, 5, 6, 7],
            tree_node_of![1, 2, 3, 4, 5, 6, 7]
        ),
        true
    );
    assert_eq!(
        Solution::is_same_tree(
            tree_node_of![1, 2, 3, 2, 5, 6, 7],
            tree_node_of![1, 2, 3, 4, 5, 6, 7]
        ),
        false
    );
    assert_eq!(
        Solution::is_same_tree(
            tree_node_of![1, 2, 3, 4, 5, -1, 7],
            tree_node_of![1, 2, 3, 4, 5, -1, 7]
        ),
        true
    );
    assert_eq!(
        Solution::is_same_tree(
            tree_node_of![1, 2, 3, -1, -1, 7],
            tree_node_of![1, 2, 3, 4, -1, -1, -1]
        ),
        false
    );
}
