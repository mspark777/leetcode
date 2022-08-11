"""
main
"""

from __future__ import annotations
from typing import Optional

class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        pre: Optional[TreeNode] = None
        stack: list[TreeNode] = []

        while root is not None or len(stack) > 0:
            while root is not None:
                stack.append(root)
                root = root.left

            root = stack.pop()
            if pre is not None and root.val <= pre.val:
                return False

            pre = root
            root = root.right

        return True


class Input:
    root: Optional[TreeNode]
    def __init__(self, root: Optional[TreeNode]):
        self.root = root

def main():
    inputs: list[Input] = [
            Input(
                TreeNode(2,
                    TreeNode(1),
                    TreeNode(3)
                )
            ),
            Input(
                TreeNode(4,
                    TreeNode(1),
                    TreeNode(4,
                        TreeNode(3),
                        TreeNode(6)
                    )
                )
            ),
    ]

    s = Solution()
    for i in inputs:
        result = s.isValidBST(i.root)
        print(result)

if __name__ == "__main__":
    main()
