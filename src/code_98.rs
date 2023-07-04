use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use Base::{tree_node::*, tree_node_of};

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let last_element = RefCell::<Option<Rc<RefCell<TreeNode>>>>::new(None);

        if root.is_some() && !Solution::in_order_visit(root, &last_element) {
            return false;
        }

        return true;
    }

    fn in_order_visit(
        node: Option<Rc<RefCell<TreeNode>>>,
        last_element: &RefCell<Option<Rc<RefCell<TreeNode>>>>,
    ) -> bool {
        match node {
            None => true,

            Some(node) => {
                let node_ref = node.borrow();
                if !Solution::in_order_visit(node_ref.left.clone(), last_element) {
                    return false;
                }
               
               let last_element_clone = last_element.borrow().clone();

                let result = match last_element_clone {
                    None => {
                        last_element.replace(Some(node.clone()));
                        Solution::in_order_visit(node_ref.right.clone(), last_element)
                    },
                    Some(element) => {
                        if element.borrow().val >= node_ref.val {
                            false
                        } else {
                            last_element.replace(Some(node.clone()));
                            Solution::in_order_visit(node_ref.right.clone(), last_element)
                        }
                    }
                };
                result
            }
        }
    }
}

#[test]
fn test_code_98() {
    assert_eq!(Solution::is_valid_bst(tree_node_of![2, 1, 3]), true);
    assert_eq!(
        Solution::is_valid_bst(tree_node_of![5, 1, 4, -1, -1, 3, 6]),
        false,
    );
    assert_eq!(
        Solution::is_valid_bst(tree_node_of![1, -1, 1]),
        false,
    );
}
