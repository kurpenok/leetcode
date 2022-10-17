#!/usr/bin/env python3

class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = s.lower()

        word = []
        symbols = "qwertyuiopasdfghjklzxcvbnm1234567890"

        for i in s:
            if i in symbols:
                word.append(i)

        s = "".join(word)

        return s == s[::-1]
