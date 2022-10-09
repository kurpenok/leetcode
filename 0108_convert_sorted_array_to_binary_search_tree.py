#!/usr/bin/env python3

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        if not len(nums):
            return None

        index = len(nums) // 2
        root = TreeNode(nums[index])

        root.left = self.sortedArrayToBST(nums[:index])
        root.right = self.sortedArrayToBST(nums[index + 1:])

        return root
