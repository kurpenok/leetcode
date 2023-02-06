class Solution:
    def romanToInt(self, s: str) -> int:
        codes = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000,
        }

        n = codes[s[0]]

        for i in range(1, len(s)):
            if codes[s[i - 1]] < codes[s[i]]:
                n += codes[s[i]] - (2 * codes[s[i - 1]])
            else:
                n += codes[s[i]]

        return n

