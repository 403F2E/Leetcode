# Given the head of a singly linked list, reverse the list, and return the reversed list.

from typing import Optional


''' ListNode class that is the base structure for the problem, with static method that returns listnode from vec '''
class ListNode:
    def __init__(self, val=0, next=None) -> None:
        self.val = val
        self.next = next

    @staticmethod
    def From(vec: list):
        if not vec:
            return None

        head = ListNode(vec[0])
        head.next = ListNode.From(vec[1::]) if len(vec) > 1 else None


        return head

''' Solution class that have the solution method and the printing method '''
class Solution:

    @staticmethod
    def reverseList(head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return None
        curr, prev = head, None

        while curr:
            next_node = curr.next
            curr.next = prev
            prev = curr
            curr = next_node

        return prev # pyright: ignore[reportReturnType]

    @staticmethod
    def print_vec(node) -> None:
        if node is None:
            print("[]")
            return 

        msg = f"[ {node.val}"
        curr = node.next
        while curr:
            msg += f" ,{curr.val}"
            curr = curr.next

        msg += " ]"
        print(msg)

if __name__=='__main__':
    example1 = ListNode.From([1, 2, 3, 4, 5])
    example2 = ListNode.From([1, 2])
    example3 = ListNode.From([3])
    example4 = ListNode.From([])

    res1 = Solution.reverseList(example1)
    res2 = Solution.reverseList(example2)
    res3 = Solution.reverseList(example3)
    res4 = Solution.reverseList(example4)

    Solution.print_vec(res1) # type: ignore
    Solution.print_vec(res2) # type: ignore
    Solution.print_vec(res3) # type: ignore
    Solution.print_vec(res4) # type: ignore
