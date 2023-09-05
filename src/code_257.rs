use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use Base::tree_node_of;
use crate::Solution;

impl Solution {
    fn binary_tree_paths_internal(node: Option<Rc<RefCell<TreeNode>>>, current: &mut Vec<String>, result: &mut Vec<String>) {
        match node {
            None => {}
            Some(node) => {
                let node_ref = node.borrow();
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    current.push(node_ref.val.to_string());
                    result.push(current.join("->"));
                    current.pop();
                } else {
                    if node_ref.left.is_some() {
                        current.push(node_ref.val.to_string());
                        Solution::binary_tree_paths_internal(node_ref.left.clone(), current, result);
                        current.pop();
                    }

                    if node_ref.right.is_some() {
                        current.push(node_ref.val.to_string());
                        Solution::binary_tree_paths_internal(node_ref.right.clone(), current, result);
                        current.pop();
                    }
                }
            }
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Vec::new();
        Solution::binary_tree_paths_internal(root, &mut vec![], &mut result);
        result
    }
}

#[test]
fn test_code_257() {
    println!("{:?}", Solution::binary_tree_paths(tree_node_of![1,2,3,4,5,6,7]));
}