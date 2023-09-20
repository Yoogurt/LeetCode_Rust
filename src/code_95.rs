use std::cell::{Ref, RefCell};
use std::rc::Rc;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    fn clone_trees(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => {
                None
            }

            Some(node) => {
                let mut new_node = TreeNode::new(node.borrow().val);
                new_node.left = Solution::clone_trees(node.borrow().left.clone());
                new_node.right = Solution::clone_trees(node.borrow().right.clone());
                Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }


    fn insert_into_node(target: i32,
                        parent: Rc<RefCell<TreeNode>>) {
        let val = parent.borrow().val;

        let mut node = TreeNode::new(target);
        if target > val {
            let child_node = parent.borrow_mut().right.take();
            match child_node {
                Some(child_node) => {
                    if child_node.borrow().val > target {
                        node.right = Some(child_node);
                    } else {
                        node.left = Some(child_node);
                    }
                }
                None => {}
            }

            parent.borrow_mut().right = Some(Rc::new(RefCell::new(node)));
        } else {
            let child_node = parent.borrow_mut().left.take();
            match child_node {
                Some(child_node) => {
                    if child_node.borrow().val > target {
                        node.right = Some(child_node);
                    } else {
                        node.left = Some(child_node);
                    }
                }
                None => {}
            }

            parent.borrow_mut().left = Some(Rc::new(RefCell::new(node)));
        }
    }

    fn visit_child_and_insert(target: i32,
                              node: Rc<RefCell<TreeNode>>,
                              result: &mut Vec<Rc<RefCell<TreeNode>>>) {

    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        }

        let mut result = vec![];

        // as root

        let last_result = Solution::generate_trees(n - 1);

        last_result.iter().for_each(|node| {
            match node {
                None => {}
                node => {
                    let mut root = TreeNode::new(n);
                    root.left = Solution::clone_trees(node.clone());
                    result.push(Rc::new(RefCell::new(root)));
                }
            }
        });


        result.into_iter().map(|value| Some(value)).collect()
    }
}

#[test]
fn test_code_95() {
    dbg!(Solution::generate_trees(2));
}