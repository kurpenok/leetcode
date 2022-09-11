from typing import Optional


class ListNode:
    def __init__(self, val=0, nxt=None) -> None:
        self.val = val
        self.next = nxt


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        numbers = []
        while head is not None:
            if not numbers:
                numbers.append(head.val)
            elif head.val != numbers[-1]:
                numbers.append(head.val)
            head = head.next

        if not numbers:
            return None

        list_node = ListNode()
        start_node = list_node
        for i, number in enumerate(numbers):
            list_node.val = number
            if i == len(numbers) - 1:
                list_node.next = None
            else:
                list_node.next = ListNode() 
                list_node = list_node.next

        return start_node
