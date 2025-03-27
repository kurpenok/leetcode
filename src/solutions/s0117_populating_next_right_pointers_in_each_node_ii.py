from __future__ import annotations

from collections import deque


class Node:
    def __init__(
        self,
        val: int = 0,
        left: Node | None = None,
        right: Node | None = None,
        next: Node | None = None,
    ):
        self.val: int = val
        self.left: Node | None = left
        self.right: Node | None = right
        self.next: Node | None = next


class Solution:
    def connect(self, root: Node | None) -> Node | None:
        if not root:
            return None

        q = deque()
        q.append(root)

        dummy = Node(-999)

        while q:
            prev = dummy
            for _ in range(len(q)):
                popped = q.popleft()

                if popped.left:
                    q.append(popped.left)
                    if prev:
                        prev.next = popped.left
                        prev = prev.next

                if popped.right:
                    q.append(popped.right)
                    if prev:
                        prev.next = popped.right
                        prev = prev.next

        return root
