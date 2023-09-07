use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    fn sum_of_left_leaves_internal(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match root {
            None => { 0 }
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                if left.is_none() && right.is_none() {
                    if is_left {
                        return node.borrow().val;
                    }

                    return 0;
                }


                Solution::sum_of_left_leaves_internal(left, true)
                    + Solution::sum_of_left_leaves_internal(right, false)
            }
        }
    }

    fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.unwrap();

        return Solution::sum_of_left_leaves_internal(node.borrow().left.clone(), true)
            + Solution::sum_of_left_leaves_internal(node.borrow().right.clone(), false);
    }
}

#[test]
fn test_code_404() {
    println!("{}", Solution::sum_of_left_leaves(tree_node_of![3,-1, 1]));
}