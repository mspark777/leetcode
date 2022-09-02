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


class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> list[float]:
        result: list[float] = []
        if root is None:
            return result

        queue: list[TreeNode] = [root]
        while len(queue) > 0:
            size = len(queue)
            total = 0.0
            for i in range(size):
                head = queue.pop(0)
                total += float(head.val)

                left = head.left
                if left is not None:
                    queue.append(left)

                right = head.right
                if right is not None:
                    queue.append(right)
            result.append(total / size)

        return result


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))),
        Input(TreeNode(3, TreeNode(9, TreeNode(15), TreeNode(7)), TreeNode(20))),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.averageOfLevels(root)
        print(result)


if __name__ == "__main__":
    main()
