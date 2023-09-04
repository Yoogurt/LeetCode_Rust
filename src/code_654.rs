use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    fn construct_internal(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let max = nums.iter().enumerate().max_by(|left, right| left.1.cmp(right.1));
        match max {
            Some(max) => {
                let node = Rc::new(RefCell::new(TreeNode::new(*max.1)));
                node.borrow_mut().left = Solution::construct_internal(&nums[0..max.0]);
                if max.0 < nums.len() - 1 {
                    node.borrow_mut().right = Solution::construct_internal(&nums[max.0 + 1..nums.len()]);
                }
                Some(node)
            }
            _ => {
                None
            }
        }
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::construct_internal(&nums)
    }
}

#[test]
fn test_code_654() {
    println!("{:?}", Solution::construct_maximum_binary_tree(vec![3, 2, 1]))
}