use Base::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let mut l1 = l1;
        let mut l2 = l2;

        let mut result = Some(Box::new(ListNode { val: 0, next: None }));
        let mut borrowed_result: Option<&mut Box<ListNode>> = None;

        let mut carry = 0;

        while l1.is_some() && l2.is_some() {
            if borrowed_result.is_none() {
                borrowed_result = result.as_mut();
            } else {
                let mut_list_node = borrowed_result.unwrap();
                if mut_list_node.next.as_mut().is_none() {
                    mut_list_node.next = Some(Box::new(ListNode::new(0)));
                }
                borrowed_result = mut_list_node.next.as_mut();
            }

            let unwrap_l1 = l1.unwrap();
            let unwrap_l2 = l2.unwrap();

            let ret = unwrap_l1.val + unwrap_l2.val + carry;

            borrowed_result.as_mut().unwrap().val = ret % 10;
            carry = ret / 10;

            l1 = unwrap_l1.next;
            l2 = unwrap_l2.next;
        }

        while l1.is_some() {
            if borrowed_result.is_none() {
                borrowed_result = result.as_mut();
            } else {
                let mut_list_node = borrowed_result.unwrap();
                if mut_list_node.next.as_mut().is_none() {
                    mut_list_node.next = Some(Box::new(ListNode::new(0)));
                }
                borrowed_result = mut_list_node.next.as_mut();
            }

            let unwrap_l1 = l1.unwrap();

            let ret = unwrap_l1.val + carry;
            //
            borrowed_result.as_mut().unwrap().val = ret % 10;

            carry = ret / 10;

            l1 = unwrap_l1.next;
        }

        while l2.is_some() {
            if borrowed_result.is_none() {
                borrowed_result = result.as_mut();
            } else {
                let mut_list_node = borrowed_result.unwrap();
                if mut_list_node.next.as_mut().is_none() {
                    mut_list_node.next = Some(Box::new(ListNode::new(0)));
                }
                borrowed_result = mut_list_node.next.as_mut();
            }

            let unwrap_l2 = l2.unwrap();

            let ret = unwrap_l2.val + carry;
            //
            borrowed_result.as_mut().unwrap().val = ret % 10;

            carry = ret / 10;

            l2 = unwrap_l2.next;
        }

        while carry > 0 {
            if borrowed_result.is_none() {
                borrowed_result = result.as_mut();
            } else {
                let mut_list_node = borrowed_result.unwrap();
                if mut_list_node.next.as_mut().is_none() {
                    mut_list_node.next = Some(Box::new(ListNode::new(0)));
                }
                borrowed_result = mut_list_node.next.as_mut();
            }

            let ret = carry;
            //
            borrowed_result.as_mut().unwrap().val = ret % 10;

            carry = ret / 10;
        }

        return result;
    }
}