#!/usr/bin/env python3

from typing import List, Optional


class ListNode:
    def __init__(self, x) -> None:
        self.val = x
        self.next = None


class Solution:
    def getListNodeValues(self, node: Optional[ListNode], values: List[int]) -> None:
        if node is None:
            return
        values.append(node.val)
        self.getListNodeValues(node.next, values)

    def getIntersectionNode(self, headA: Optional[ListNode], headB: Optional[ListNode]) -> Optional[ListNode]:
        start_a = headA
        start_b = headB

        while start_a != start_b:
            if start_a:
                start_a = start_a.next
            else:
                start_a = headB

            if start_b:
                start_b = start_b.next
            else:
                start_b = headA

        return start_a
