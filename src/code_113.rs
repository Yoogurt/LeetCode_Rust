use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    fn path_sum_internal(target_sum: i32,
                         current: Option<Rc<RefCell<TreeNode>>>,
                         current_path: &mut Vec<i32>,
                         result: &mut Vec<Vec<i32>>) {
        match current {
            None => {
                if target_sum == 0 {
                    result.push(current_path.clone());
                }
            }
            Some(current) => {
                let val = current.borrow().val;
                let target_sum = target_sum - val;
                current_path.push(val);

                let left = &current.borrow().left;
                let right = &current.borrow().right;

                if left.is_none() && right.is_none() {
                    if target_sum == 0 {
                        result.push(current_path.clone());
                    }
                }

                if left.is_some() {
                    Solution::path_sum_internal(target_sum, left.clone(), current_path, result);
                }
                if right.is_some() {
                    Solution::path_sum_internal(target_sum, right.clone(), current_path, result);
                }
                current_path.pop();
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Solution::path_sum_internal(target_sum, root, &mut vec![], &mut result);
        result
    }
}

#[test]
fn test_code_113() {
    dbg!(Solution::path_sum(tree_node_of![5,4,8,11,-1,13,4,7,2,-1,-1,5,1], 22));
}