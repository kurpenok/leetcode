from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        insert_index = 0
        for i in range(m):
            if nums2[insert_index] < nums1[i]:
                nums2[]
