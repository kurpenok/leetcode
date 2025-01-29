from __future__ import annotations


class Node:
    def __init__(self, val: int | None = None, children: list[Node] | None = None):
        self.val = val
        self.children = children


def maxDepth(self, root: Node | None) -> int:
    if not root:
        return 0

    max_depth = 0
    if root.children:
        for child in root.children:
            max_depth = max(max_depth, self.maxDepth(child))

    return max_depth + 1
