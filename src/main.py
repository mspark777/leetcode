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


def newnode(val: int, left: Optional[TreeNode], right: Optional[TreeNode]) -> TreeNode:
    return TreeNode(val, left, right)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


def newleft(val: int, left: Optional[TreeNode]) -> TreeNode:
    return newnode(val, left, None)


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return newnode(val, None, right)


class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> list[int]:
        stack: list[TreeNode] = []
        result: list[int] = []
        top = root
        while (top is not None) or stack:
            while top is not None:
                stack.append(top)
                top = top.left

            top = stack.pop()
            result.append(top.val)
            top = top.right

        return result


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(newright(1, newleft(2, newval(3)))),
        Input(None),
        Input(newval(1)),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.inorderTraversal(root)
        print(result)


if __name__ == "__main__":
    main()
