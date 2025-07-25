# Given head, the head of a linked list, determine if the linked list has a cycle in it.
#
# There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
#
# Return true if there is a cycle in the linked list. Otherwise, return false.

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next: Optional[ListNode] = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]):
        f = head
        while f and f.next:
            head = head.next
            f = f.next.next
            if head == f: return True

        return False
