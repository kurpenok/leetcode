class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        indexes = dict()

        for i, num in enumerate(nums):
            if num == val:
                indexes[i] = True
        
        start = 0
        for i in range(len(nums)):
            if i in indexes:
                start += 1
            else:
                nums[i - start] = nums[i]

        return len(nums) - len(indexes)

