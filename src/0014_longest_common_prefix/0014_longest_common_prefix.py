class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ""
        
        for symbols in zip(*strs):
            start = symbols[0]
            for symbol in symbols:
                if symbol != start:
                    return prefix
            prefix += start
        
        return prefix

