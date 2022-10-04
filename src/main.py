from __future__ import annotations
from typing import Optional, List


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


def newleft(val: int, left: Optional[TreeNode]) -> TreeNode:
    return newnode(val, left, None)


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return newnode(val, None, right)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


class Solution:
    def hasPathSum(self, root: Optional[TreeNode], target_sum: int) -> bool:
        if root is None:
            return False

        stack: list[tuple[TreeNode, int]] = [(root, target_sum)]
        while stack:
            (node, target) = stack.pop()
            left = node.left
            right = node.right
            newval = target - node.val
            isleaf = True

            if left is not None:
                isleaf = False
                stack.append((left, newval))

            if right is not None:
                isleaf = False
                stack.append((right, newval))

            if isleaf and (newval == 0):
                return True

        return False


class Input:
    root: Optional[TreeNode]
    target_sum: int

    def __init__(self, root: Optional[TreeNode], target_sum: int) -> None:
        self.root = root
        self.target_sum = target_sum


def main():
    inputs: list[Input] = [
        Input(
            newnode(
                5,
                newleft(4, newnode(11, newval(7), newval(2))),
                newnode(8, newval(13), newright(4, newval(1))),
            ),
            22,
        ),
        Input(
            newnode(5, newval(2), newval(3)),
            5,
        ),
        Input(None, 0),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        target_sum = input.target_sum
        result = solution.hasPathSum(root, target_sum)
        print(result)


if __name__ == "__main__":
    main()
