#! /usr/bin/python3

from typing import List


class Solution:
    @staticmethod
    def plusOne(digits: List[int]) -> List[int]:
        index = len(digits) - 1

        while index >= 0:
            digits[index] = (digits[index] + 1) % 10
            if digits[index]:
                break
            index -= 1

        if not digits[0]:
            temp = [0] * (len(digits) + 1)
            temp[0] = 1
            for i in range(len(digits)):
                temp[i + 1] = digits[i]
            digits = temp

        return digits


if __name__ == "__main__":
    print(Solution.plusOne([1, 2, 3]))
    print(Solution.plusOne([4, 3, 2, 1]))
    print(Solution.plusOne([9]))
