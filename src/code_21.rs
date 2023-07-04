use std::arch::aarch64::vdup_n_p16;
use LeetCodeBase::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut result = Box::new(ListNode::new(-1));
        let mut ref_result = &mut result;

        while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
            ref_result.next = Some(Box::new(ListNode::new(if n1.val < n2.val {
                let ret = n1.val;
                list1 = list1.unwrap().next;
                ret
            } else {
                let ret = n2.val;
                list2 = list2.unwrap().next;
                ret
            })));

            ref_result = ref_result.next.as_mut().unwrap();
        }

        while let Some(n2) = list2.as_ref() {
            ref_result.next = Some(Box::new(ListNode::new({
                let ret = n2.val;
                list2 = list2.unwrap().next;
                ret
            })));

            ref_result = ref_result.next.as_mut().unwrap();
        }

        while let Some(n1) = list1.as_ref() {
            ref_result.next = Some(Box::new(ListNode::new({
                let ret = n1.val;
                list1 = list1.unwrap().next;
                ret
            })));

            ref_result = ref_result.next.as_mut().unwrap();
        }

        return result.next;
    }
}