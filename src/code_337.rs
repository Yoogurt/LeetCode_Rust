use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    fn rob_internal(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => {
                (0, 0)
            }
            Some(node) => {
                let left_rob = Solution::rob_internal(node.borrow().left.clone());
                let right_rob = Solution::rob_internal(node.borrow().right.clone());

                (node.borrow().val + left_rob.1 + right_rob.1,
                 left_rob.0.max(left_rob.1) + right_rob.0.max(right_rob.1))
            }
        }
    }

    pub fn rob_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Solution::rob_internal(root);

        result.0.max(result.1)
    }
}

#[test]
fn test_code_337() {
    // println!("{:?}", Solution::rob_2(tree_node_of![3,2,3,-1,3,-1,1]));
    println!("{:?}", Solution::rob_2(tree_node_of![ 4,1, -1, 2, -1, 3]));
}