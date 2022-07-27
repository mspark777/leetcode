"""
solution
"""
from __future__ import annotations
from typing import Optional

class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]
    def __init__(self, val: int, left: Optional[TreeNode], right: Optional[TreeNode]):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        while root is not None:
            if root.left is not None:
                right = root.right
                predecessor = root.left
                while predecessor.right is not None:
                    predecessor = predecessor.right

                predecessor.right = right
                root.right = root.left
                root.left = None

            root = root.right
