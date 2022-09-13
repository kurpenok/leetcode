#! /usr/bin/python3.10

from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        insert_index = 0
        for i in range(n + m):
            if nums2[insert_index] < nums1[i]:
                for j in range(n + m - 1, i + 1, -1):
                    nums1[j] = nums1[j - 1]
                nums1[i] = nums2[insert_index]
                insert_index += 1
