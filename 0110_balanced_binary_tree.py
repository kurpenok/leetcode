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

        ldepth = self.maxDepth(root.left)
        if ldepth == -1:
            return -1

        rdepth = self.maxDepth(root.right)
        if rdepth == -1:
            return -1

        if abs(ldepth - rdepth) > 1:
            return -1
        return max(ldepth, rdepth) + 1

    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        return self.maxDepth(root) != -1
