class Solution:
    def isValid(self, s: str) -> bool:
        braces = {")": "(", "]": "[", "}": "{"}
        stack = []

        for brace in s:
            if brace in "({[":
                stack.append(brace)
            else:
                if not stack:
                    return False
                pop = stack.pop()
                if pop != braces[brace]:
                    return False

        if stack:
            return False
        return True

