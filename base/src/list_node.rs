use std::fmt::{Display, Formatter};
use std::slice::Iter;
use std::vec::Vec;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.next == other.next
    }
}

pub struct ListNodesFomatter<'a> {
    pub list_node: &'a Option<Box<ListNode>>,
}

impl Display for ListNodesFomatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        format_list_node(f, self.list_node)?;
        f.write_str("}")?;
        return Ok(());
    }
}

fn format_list_node(f: &mut Formatter<'_>, list_node: &Option<Box<ListNode>>) -> std::fmt::Result {
    if list_node.is_none() { return Ok(()); }
    let b_l = list_node.as_ref().unwrap();
    f.write_str(b_l.val.to_string().as_str()).unwrap();
    f.write_str(",").unwrap();
    return format_list_node(f, &b_l.next);
}

pub fn list_node_of_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }

    let iter = &mut values.iter();
    return rec_list_node_of(iter);
}

fn rec_list_node_of(iter: &mut Iter<i32>) -> Option<Box<ListNode>> {
    let next = iter.next();

    match next {
        None => {
            return None;
        }
        Some(&val) => {
            Some(Box::new(ListNode {
                val,
                next: rec_list_node_of(iter),
            }))
        }
    }
}

#[macro_export]
macro_rules! list_node_of {
    [$($x:expr),*] => {
        {
            list_node_of_vec(vec![$($x),*])
        }
    };
}