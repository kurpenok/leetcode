class Solution:
    def climbStairs(self, n: int) -> int:
        jumps = {1: 1, 2: 2}
        
        for i in range(3, n + 1):
            jumps[i] = jumps[i - 1] + jumps[i - 2]
            
        return jumps[n]

