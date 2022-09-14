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


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return newnode(val, None, right)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


class StackNode:
    node: Optional[TreeNode]
    path: int

    def __init__(self, node: Optional[TreeNode], path: int) -> None:
        self.node = node
        self.path = path


class Solution:
    def pseudoPalindromicPaths(self, root: Optional[TreeNode]) -> int:
        result = 0
        stack: list[StackNode] = [StackNode(root, 0)]

        while stack:
            top = stack.pop()
            node = top.node
            if node is None:
                continue

            val = node.val
            left = node.left
            right = node.right
            path = top.path ^ (1 << val)
            if (left is None) and (right is None):
                if (path & (path - 1)) == 0:
                    result += 1
            else:
                stack.append(StackNode(left, path))
                stack.append(StackNode(right, path))

        return result


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(newnode(2, newnode(3, newval(3), newval(1)), newright(1, newval(1)))),
        Input(newnode(2, newnode(1, newval(1), newright(3, newval(1))), newval(1))),
        Input(newval(9)),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.pseudoPalindromicPaths(root)
        print(result)


if __name__ == "__main__":
    main()
