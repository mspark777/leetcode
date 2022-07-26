"""
solution
"""
from __future__ import annotations
from typing import Optional

class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]
    def __init__(self, x: int, left: Optional[TreeNode], right: Optional[TreeNode]):
        self.val = x
        self.left = left
        self.right = right

class Solution:
    def lowestCommonAncestor(self, root: Optional[TreeNode], p: Optional[TreeNode], q: Optional[TreeNode]) -> Optional[TreeNode]:
        if (root is None) or (root is p) or (root is q):
            return root

        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)

        if left is None:
            return right
        elif right is None:
            return left
        else:
            return root
