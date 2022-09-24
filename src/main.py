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


def newleft(val: int, left: Optional[TreeNode]) -> TreeNode:
    return newnode(val, left, None)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


class StackNode:
    path: list[int]
    node: TreeNode
    sum: int

    def __init__(self, path: list[int], node: TreeNode, sum: int) -> None:
        self.path = path
        self.node = node
        self.sum = sum


class Solution:
    def pathSum(self, root: Optional[TreeNode], target_sum: int) -> list[list[int]]:
        if root is None:
            return []

        result: list[list[int]] = []
        stack: list[StackNode] = [StackNode([], root, 0)]

        while stack:
            top = stack.pop()
            path = top.path
            sum = top.sum
            node = top.node
            val = node.val
            left = node.left
            right = node.right
            newsum = sum + val

            is_leaf = True

            if left is not None:
                is_leaf = False
                newpath = path.copy()
                newpath.append(val)
                stack.append(StackNode(newpath, left, newsum))

            if right is not None:
                is_leaf = False
                newpath = path.copy()
                newpath.append(val)
                stack.append(StackNode(newpath, right, newsum))

            if is_leaf and (newsum == target_sum):
                newpath = path.copy()
                newpath.append(val)
                result.append(newpath)

        return result


class Input:
    root: Optional[TreeNode]
    target_sum: int

    def __init__(self, root: Optional[TreeNode], target_sum: int):
        self.root = root
        self.target_sum = target_sum


def main():
    inputs: list[Input] = [
        Input(
            newnode(
                5,
                newleft(4, newnode(11, newval(7), newval(2))),
                newnode(8, newval(13), newnode(4, newval(5), newval(1))),
            ),
            22,
        ),
        Input(newnode(1, newval(2), newval(3)), 5),
        Input(newleft(1, newval(2)), 0),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        target_sum = input.target_sum
        result = solution.pathSum(root, target_sum)
        print(result)


if __name__ == "__main__":
    main()
