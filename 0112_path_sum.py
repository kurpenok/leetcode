#!/usr/bin/env python3

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False

        targetSum -= root.val

        if root.left is None and root.right is None and not targetSum:
            return True

        return self.hasPathSum(root.left, targetSum) or self.hasPathSum(root.right, targetSum)
