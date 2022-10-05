#!/usr/bin/env python3

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        depth = 0

        # queue = []
        # queue.append(root)

        # while queue:
        #     node = queue.pop()
        #     if node.left is not None:
        #         queue.append(node.left)
        #     if node.right is not None:
        #         queue.append(node.right)
        #     depth += 1

        ldepth = self.maxDepth(root.left)
        rdepth = self.maxDepth(root.right)

        if ldepth > rdepth:
            return ldepth + 1
        else:
            return rdepth + 1

        # return depth
