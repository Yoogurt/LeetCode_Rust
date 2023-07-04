use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
    
        let mut queue = Vec::<Rc<RefCell<TreeNode>>>::new();
        let root = Rc::<RefCell<TreeNode>>::new(RefCell::new(TreeNode::new(*data.first()?)));
        queue.push(root.clone());
    
        let mut index = 1;
        while index < data.len() {
            let option_node = queue.remove(0);
            let mut node = option_node.borrow_mut();
    
            let left = if data[index] != -1 {
                let result = Rc::new(RefCell::new(TreeNode::new(data[index])));
                queue.push(result.clone());
                Some(result)
            } else {
                None
            };
            node.left = left;
            index += 1;
    
            if index < data.len() {
                let right = if data[index] != -1 {
                    let result = Rc::new(RefCell::new(TreeNode::new(data[index])));
                    queue.push(result.clone());
                    Some(result)
                } else {
                    None
                };
                node.right = right;
                index += 1;
            } else {
                break;
            }
        }
    
        return Some(root);
    }
}

#[macro_export]
macro_rules! tree_node_of {
    [$($x:expr),*] => {
        {
            Base::tree_node::TreeNode::from_vec(vec![$($x),*])
        }
    };
}