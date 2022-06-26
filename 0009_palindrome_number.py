class Solution:
    def isPalindrome(self, x: int) -> bool:
        x = str(x)
        middle = len(x) // 2

        if len(x) % 2:
            return x[:middle + 1] == (x[middle:])[::-1]
        else:
            return x[:middle] == (x[middle:])[::-1]

