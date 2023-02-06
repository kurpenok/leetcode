#! /usr/bin/python3


class Solution:
    @staticmethod
    def addBinary(a: str, b: str) -> str:
        return bin(int(int(a, 2) + int(b, 2)))[2:]


if __name__ == "__main__":
    print(Solution.addBinary("11", "1"))
    print(Solution.addBinary("1010", "1011"))
