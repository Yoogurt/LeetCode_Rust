use std::cell::RefCell;
use std::rc::Rc;
use Base::tree_node::TreeNode;
use crate::Solution;

impl Solution {
    fn sorted_array_to_bst_internal(nums: &Vec<i32>, left: usize, right: usize)
                                    -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let middle = (left + right) >> 1;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[middle])));
        {
            let mut node_mut = node.borrow_mut();
            if middle > 0 {
                node_mut.left = Solution::sorted_array_to_bst_internal(nums, left, middle - 1);
            }

            node_mut.right = Solution::sorted_array_to_bst_internal(nums, middle + 1, right);
        }

        Some(node)
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        Solution::sorted_array_to_bst_internal(&nums, 0, nums.len() - 1)
    }
}

#[test]
fn test_code_108() {
    println!("{:?}", Solution::sorted_array_to_bst(vec![-10,-3,0,5,9]))
}