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
    def tree2str(self, root: Optional[TreeNode]) -> str:
        if root is None:
            return ""

        stack: list[TreeNode] = [root]
        visiteds: set[TreeNode] = set()
        result: list[str] = []
        while stack:
            node = stack[-1]
            if node in visiteds:
                stack.pop()
                result.append(")")
                continue

            visiteds.add(node)
            result.append("(")
            result.append(str(node.val))

            left = node.left
            right = node.right
            if (left is None) and (right is not None):
                result.append("()")

            if right is not None:
                stack.append(right)

            if left is not None:
                stack.append(left)

        return "".join(result[1:-1])


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(newnode(1, newleft(2, newval(4)), newval(3))),
        Input(newnode(1, newright(2, newval(4)), newval(3))),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.tree2str(root)
        print(result)


if __name__ == "__main__":
    main()
