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
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        stack: list[Optional[TreeNode]] = [root]
        while len(stack) > 0:
            node = stack.pop()
            if node is None:
                continue

            left = node.left
            right = node.right
            node.right = left
            node.left = right
            stack.append(left)
            stack.append(right)
        return root


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root

    def travel(self, node: Optional[TreeNode]):
        if node is not None:
            self.travel(node.left)
            print(node.val, end=" ")
            self.travel(node.right)


def main():
    inputs: list[Input] = [
        Input(
            TreeNode(
                4,
                TreeNode(2, TreeNode(1), TreeNode(3)),
                TreeNode(7, TreeNode(6), TreeNode(9)),
            )
        ),
        Input(TreeNode(2, TreeNode(1), TreeNode(3))),
        Input(None),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.invertTree(root)
        input.travel(result)
        print("")


if __name__ == "__main__":
    main()
