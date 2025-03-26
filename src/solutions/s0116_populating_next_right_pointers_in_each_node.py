from __future__ import annotations


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

        left_most = root

        while left_most.left:
            current_node = left_most

            while current_node:
                if current_node.left:
                    current_node.left.next = current_node.right
                    if current_node.next and current_node.right:
                        current_node.right.next = current_node.next.left
                    current_node = current_node.next

            left_most = left_most.left

        return root
