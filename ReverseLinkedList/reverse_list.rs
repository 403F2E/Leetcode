// Given the head of a singly linked list, reverse the list, and return the reversed list.

#![allow(dead_code, unused)]

use std::convert::From;
use std::fmt;

/* ListNode base Structure for the problem */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(list_val: Vec<i32>) -> ListNode {
        if list_val.is_empty() {
            return ListNode::new(5001);
        }

        let mut head = ListNode::new(list_val[0]);
        head.next = if list_val.len() > 1 {
            Some(Box::new(ListNode::from(list_val[1..].to_vec())))
        } else {
            None
        };

        head
    }
}

/* The 2 Solutions: using loop and recursivity */

type Node = Option<Box<ListNode>>;

pub fn reverse_list(head: Node) -> Node {
    let (mut curr, mut prev) = (head, None);

    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node)
    }

    prev
}

fn reverse_recursive(head: Node) -> Node {
    fn recursive(head: Node, prev: Node) -> Node {
        match head {
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                recursive(next, Some(node))
            }
            None => prev,
        }
    }
    recursive(head, None)
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.val == 5001 {
            return write!(f, "[]");
        }
        write!(f, "[ ");
        write!(f, "{}", self.val);

        let mut curr = self.clone().next;
        while let Some(mut node) = curr {
            write!(f, " ,{}", node.val);
            curr = node.next;
        }
        write!(f, " ]");
        Ok(())
    }
}

fn main() {
    let example1 = ListNode::from(vec![1, 2, 3, 4, 5]);
    let example2 = ListNode::from(vec![1, 2]);
    let example3 = ListNode::from(vec![3]);
    let example4 = ListNode::from(vec![]);

    let res1 = reverse_list(Some(Box::new(example1)));
    let res2 = reverse_list(Some(Box::new(example2)));
    let res3 = reverse_list(Some(Box::new(example3)));
    let res4 = reverse_list(Some(Box::new(example4)));

    println!("{}", res1.unwrap());
    println!("{}", res2.unwrap());
    println!("{}", res3.unwrap());
    println!("{}", res4.unwrap());
}
