#! /usr/bin/python3

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def recursiveParser(self, point: Optional[TreeNode] | None, values: List[int]) -> None:
        if point is None:
            return

        self.recursiveParser(point.left, values)
        values.append(point.val)
        self.recursiveParser(point.right, values)

    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        values = []

        self.recursiveParser(root, values)

        return values
