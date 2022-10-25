#!/usr/bin/env python3


class Solution:
    def convertToTitle(self, columnNumber: int) -> str:
        symbols = "abcdefghijklmnopqrstuvwxyz"
        title = ""

        while columnNumber:
            title = symbols[(columnNumber % 26) - 1].upper() + title
            columnNumber -= 1
            columnNumber //= 26

        return title
