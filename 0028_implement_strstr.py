class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        sc = ""
        for i in range(len(haystack)):
            if i + len(needle) > len(haystack):
                return -1
            sc = haystack[i:i + len(needle)]
            if sc == needle:
                return i
        return -1
