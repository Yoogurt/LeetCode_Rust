use std::cell::RefCell;
use std::rc::Rc;

use Base::tree_node::TreeNode;
use Base::tree_node_of;


use crate::Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();

        if root.is_none() {
            return result;
        }
        let root = root.unwrap();

        let mut current_level = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut next_level = Vec::<Rc<RefCell<TreeNode>>>::new();
        next_level.push(root);

        while !next_level.is_empty() {
            current_level = next_level;
            next_level = Vec::<Rc<RefCell<TreeNode>>>::new();
            let next_level_ref = &mut next_level;

            let level_result = current_level
                .iter()
                .fold(Vec::<i32>::new(), |mut result, value| {
                    result.push(value.borrow().val);

                    let left_child = value.borrow().left.clone();
                    let right_child = value.borrow().right.clone();

                    if left_child.is_some() {
                        next_level_ref.push(left_child.unwrap());
                    }
                    if right_child.is_some() {
                        next_level_ref.push(right_child.unwrap());
                    }

                    result
                });

            result.push(level_result);
        }

        return result;
    }
}

#[test]
fn test_code_102() {
    assert_eq!(
        Solution::level_order(tree_node_of![3, 9, 20, -1, -1, 15, 7]),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );
    assert_eq!(Solution::level_order(tree_node_of![1]), vec![vec![1]]);
    assert_eq!(
        Solution::level_order(tree_node_of![]),
        Vec::<Vec<i32>>::new()
    );
}
