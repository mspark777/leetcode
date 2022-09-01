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
    max: int
    node: TreeNode

    def __init__(self, node: TreeNode, max: int):
        self.node = node
        self.max = max


class Solution:
    def goodNodes(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        result = 0

        stack: list[StackNode] = [StackNode(root, root.val)]
        while len(stack) > 0:
            top = stack.pop()
            node = top.node
            val = node.val
            max = top.max if top.max > val else val
            if val == max:
                result += 1

            left = node.left
            if left is not None:
                stack.append(StackNode(left, max))

            right = node.right
            if right is not None:
                stack.append(StackNode(right, max))

        return result


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(
            TreeNode(3, TreeNode(1, TreeNode(3)), TreeNode(4, TreeNode(1), TreeNode(5)))
        ),
        Input(TreeNode(3, TreeNode(3, TreeNode(4), TreeNode(2)))),
        Input(TreeNode(1)),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.goodNodes(root)
        print(result)


if __name__ == "__main__":
    main()
