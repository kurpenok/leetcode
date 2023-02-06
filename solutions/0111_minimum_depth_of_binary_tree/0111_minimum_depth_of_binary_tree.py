#!/usr/bin/env python3

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        depth = 0
        depths = []

        queue = []
        queue.append([root, 1])

        while queue:
            node, depth = queue.pop(0)

            if node.left is None and node.right is None:
                depths.append(depth)
                continue

            if node.left is not None:
                queue.append([node.left, depth + 1])
            if node.right is not None:
                queue.append([node.right, depth + 1])

        return min(depths)
