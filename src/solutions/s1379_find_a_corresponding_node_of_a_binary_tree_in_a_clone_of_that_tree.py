class TreeNode:
    def __init__(self, x: int):
        self.val: int = x
        self.left: TreeNode | None = None
        self.right: TreeNode | None = None


class Solution:
    def getTargetCopy(
        self,
        original: TreeNode | None,
        cloned: TreeNode | None,
        target: TreeNode | None,
    ) -> TreeNode | None:
        if not original or target == original:
            return cloned

        if cloned:
            return self.getTargetCopy(
                original.left, cloned.left, target
            ) or self.getTargetCopy(original.right, cloned.right, target)
        return None
