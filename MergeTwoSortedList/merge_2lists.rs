// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn merge_two_lists(list1: Node, list2: Node) -> Node {
    match (list1, list2) {
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
        (Some(n1), Some(n2)) => {
            if n1.val <= n2.val {
                return Some(Box::new(ListNode {
                    next: merge_two_lists(n1.next, Some(n2)),
                    val: n1.val,
                }));
            } else {
                return Some(Box::new(ListNode {
                    next: merge_two_lists(Some(n1), n2.next),
                    val: n2.val,
                }));
            }
        }
    }
}
