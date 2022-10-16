#!/usr/bin/env python3

from math import factorial

from typing import List


class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        row = [0 for _ in range(rowIndex + 1)]

        for i in range(len(row)):
            row[i] = int(factorial(rowIndex) / (factorial(i) * factorial(rowIndex - i)))

        return row
