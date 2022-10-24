#!/usr/bin/env python3

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def recursiveHelper(self, root: Optional[TreeNode], values: List[int]) -> None:
        if root is None:
            return

        self.recursiveHelper(root.left, values)
        self.recursiveHelper(root.right, values)
        values.append(root.val)

    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        values = []

        if root is None:
            return list(values)

        self.recursiveHelper(root.left, values)
        self.recursiveHelper(root.right, values)
        values.append(root.val)

        return list(values)
