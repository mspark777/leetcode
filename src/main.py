"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class StackNode:
    path: list[TreeNode]
    node: TreeNode

    def __init__(self, node: TreeNode, path: list[TreeNode]):
        self.node = node
        self.path = path


class Solution:
    def binaryTreePaths(self, root: Optional[TreeNode]) -> list[str]:
        if root is None:
            return []

        stack: list[StackNode] = [StackNode(root, [])]

        result: list[list[str]] = []
        while len(stack) > 0:
            top = stack.pop()
            node = top.node
            left = node.left
            right = node.right
            path = top.path
            path.append(node)

            if (left is None) and (right is None):
                result.append([str(n.val) for n in path])
                continue

            if left is not None:
                stack.append(StackNode(left, path.copy()))

            if right is not None:
                stack.append(StackNode(right, path.copy()))

        return ["->".join(p) for p in result]


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(TreeNode(1, TreeNode(2, None, TreeNode(5)), TreeNode(3))),
        Input(TreeNode(1)),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.binaryTreePaths(root)
        print(result)


if __name__ == "__main__":
    main()
