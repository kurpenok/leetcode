#!/usr/bin/env python3

from typing import List


class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        triangle = []

        for i in range(numRows):
            line = []
            index = 0

            while True:
                if i == 0:
                    triangle.append([1])
                    break
                if i == 1:
                    triangle.append([1, 1])
                    break

                if not index:
                    line.append(1)
                    index += 1
                    continue
                if i == index:
                    line.append(1)
                    triangle.append(line)
                    break

                line.append(triangle[i - 1][index - 1] + triangle[i - 1][index])

                index += 1

        return triangle
