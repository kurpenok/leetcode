#!/usr/bin/env python3

from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        stack = []
        stack.append(root.right)
        stack.append(root.left)

        while stack:
            left = stack.pop()
            right = stack.pop()

            if left is None and right is None:
                continue
            elif left is None or right is None or left.val != right.val:
                return False

            stack.append(left.left)
            stack.append(right.right)

            stack.append(left.right)
            stack.append(right.left)

        return True
