use std::cell::RefCell;
use std::rc::Rc;

use Base::{tree_node::{TreeNode}, tree_node_of};

use crate::Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();

        return Solution::is_symmetric_compare(
            root.borrow().left.clone(),
            root.borrow().right.clone(),
        );
    }

    fn is_symmetric_compare(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.clone().xor(right.clone()).is_some() {
            return false;
        }

        if left.is_none() {
            return true;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        let left_ref = left.borrow();
        let right_ref = right.borrow();

        if left_ref.val != right_ref.val {
            return false;
        }

        return Solution::is_symmetric_compare(left_ref.left.clone(), right_ref.right.clone())
            && Solution::is_symmetric_compare(left_ref.right.clone(), right_ref.left.clone());
    }
}

#[test]
fn test_code_101() {
    assert_eq!(Solution::is_symmetric(tree_node_of![1, 2, 2, 3, 4, 4, 3]), true);
    assert_eq!(Solution::is_symmetric(tree_node_of![1, 2, 2, -1, 3, -1, 3]), false);
}
