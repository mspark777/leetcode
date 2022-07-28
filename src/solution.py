"""
solution
"""
from __future__ import annotations
from typing import  Optional

class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]
    def __init__(self, val: int=0, left: Optional[TreeNode] = None, right: Optional[TreeNode] = None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> list[int]:
        if root is None:
            return []

        result: list[int] = []
        stack: list[TreeNode] = [root]

        while len(stack) > 0:
            node = stack.pop()
            result.append(node.val)

            left = node.left
            if left is not None:
                stack.append(left)

            right = node.right
            if right is not None:
                stack.append(right)

        result.reverse()
        return result
