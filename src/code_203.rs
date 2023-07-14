use Base::{list_node::*, list_node_of};

use crate::Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        loop {
            let list_node = head.as_ref();
            if list_node.is_none() {
                return None;
            }

            if list_node.unwrap().val == val {
                head = head.unwrap().next;
            } else {
                break;
            }
        }

        if head.is_none() {
            return None;
        }

        let mut current = head.as_mut().unwrap();
        loop {
            if current.next.is_none() {
                break;
            }
            let next_value = current.next.as_ref().unwrap().val;
         
            if next_value == val {
                let node = if current.next.as_ref().unwrap().next.is_none() {
                    None
                } else {
                    current.next.as_mut().unwrap().next.take()
                };

                current.next = node;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        return head;
    }
}

#[test]
fn test_code_203() {
    assert_eq!(Solution::remove_elements(list_node_of![1,2,3,4,5], 3), list_node_of!(1,2,4,5));
    assert_eq!(Solution::remove_elements(list_node_of![1,2,3,4,5], 1), list_node_of!(2,3,4,5));
    assert_eq!(Solution::remove_elements(list_node_of![1,1,3,4,5], 1), list_node_of!(3,4,5));
    assert_eq!(Solution::remove_elements(list_node_of![1,2,3,4,5], 6), list_node_of!(1,2,3,4,5));
}