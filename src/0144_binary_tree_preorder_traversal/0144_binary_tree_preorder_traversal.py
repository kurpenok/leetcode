#!/usr/bin/env python3

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def recusiveHelper(self, root: Optional[TreeNode], values: List[int]) -> None:
        if root is None:
            return

        values.append(root.val)

        self.recusiveHelper(root.left, values)
        self.recusiveHelper(root.right, values)

    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        values = []

        if root is None:
            return values

        values.append(root.val)

        self.recusiveHelper(root.left, values)
        self.recusiveHelper(root.right, values)

        return values
