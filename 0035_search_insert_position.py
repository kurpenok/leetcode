#! /usr/bin/python3

from typing import List


class Solution:
    @staticmethod
    def searchInsert(nums: List[int], target: int) -> int:
        left = 0
        right = len(nums) - 1

        while left <= right:
            middle = left + (right - left) // 2

            if target == nums[middle]:
                return middle
            elif target > nums[middle]:
                left = middle + 1
            else:
                right = middle - 1

        return left


if __name__ == "__main__":
    print(Solution.searchInsert([1, 3, 5, 6], 5))
    print(Solution.searchInsert([1, 3, 5, 6], 2))
    print(Solution.searchInsert([1, 3, 5, 6], 7))
