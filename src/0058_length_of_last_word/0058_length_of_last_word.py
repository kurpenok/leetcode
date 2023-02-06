#! /usr/bin/python3

class Solution:
    @staticmethod
    def lengthOfLastWord(s: str) -> int:
        return len(s.split()[-1])


if __name__ == "__main__":
    print(Solution.lengthOfLastWord("Hello World"))
    print(Solution.lengthOfLastWord("   fly me   to   the moon  "))
    print(Solution.lengthOfLastWord("luffy is still joyboy"))
