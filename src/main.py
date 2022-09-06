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


def preorder(node: Optional[TreeNode]):
    if node is not None:
        preorder(node.left)
        print(node.val, end=" ")
        preorder(node.right)


class Solution:
    def pruneTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        return root if self.contains_one(root) else None

    def contains_one(self, node: Optional[TreeNode]) -> bool:
        if node is None:
            return False

        left_contained = self.contains_one(node.left)
        if not left_contained:
            node.left = None

        right_contained = self.contains_one(node.right)
        if not right_contained:
            node.right = None

        return (node.val == 1) or left_contained or right_contained


class Input:
    root: Optional[TreeNode]

    def __init__(self, root: Optional[TreeNode]):
        self.root = root


def main():
    inputs: list[Input] = [
        Input(newright(1, newnode(0, newval(0), newval(1)))),
        Input(
            newnode(
                1, newnode(0, newval(0), newval(0)), newnode(1, newval(0), newval(1))
            )
        ),
        Input(
            newnode(
                1,
                newnode(1, newleft(1, newval(0)), newval(1)),
                newnode(0, newval(0), newval(1)),
            )
        ),
    ]

    solution = Solution()
    for input in inputs:
        root = input.root
        result = solution.pruneTree(root)
        preorder(result)
        print("")


if __name__ == "__main__":
    main()
