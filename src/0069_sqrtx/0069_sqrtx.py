#! /usr/bin/python3


class Solution:
    @staticmethod
    def mySqrt(x: int) -> int:
        if x == 0:
            return 0
        elif x == 1 or x == 2:
            return 1
        else:
            for i in range(x):
                if i * i == x:
                    return i
                elif i * i > x:
                    return i - 1


if __name__ == "__main__":
    print(Solution.mySqrt(4))
    print(Solution.mySqrt(8))
