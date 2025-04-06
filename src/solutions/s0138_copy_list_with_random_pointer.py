from __future__ import annotations


class Node:
    def __init__(self, x: int, next: Node | None = None, random: Node | None = None):
        self.val: int = int(x)
        self.next: Node | None = next
        self.random: Node | None = random


class Solution:
    def copyRandomList(self, head: Node | None) -> Node | None:
        if not head:
            return None
        old_to_new = {}

        curr = head
        while curr:
            old_to_new[curr] = Node(curr.val)
            curr = curr.next

        curr = head
        while curr:
            old_to_new[curr].next = old_to_new.get(curr.next)
            old_to_new[curr].random = old_to_new.get(curr.random)
            curr = curr.next

        return old_to_new[head]
