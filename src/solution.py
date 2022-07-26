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
    def preorderTraversal(self, root: Optional[TreeNode]) -> list[int]:
        result: list[int] = []
        self.preorder(root, result)
        return result

    def preorder(self, root: Optional[TreeNode], result: list[int]):
        if root is not None:
            result.append(root.val)
            self.preorder(root.left, result)
            self.preorder(root.right, result)
