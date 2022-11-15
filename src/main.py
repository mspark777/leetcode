from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


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
    def countNodes(self, root: Optional[TreeNode]) -> int:
        nodes = 0
        h = self.get_height(root)
        while root is not None:
            next = h - 1
            if self.get_height(root.right) == next:
                nodes += 1 << h
                root = root.right
            else:
                nodes += 1 << next
                root = root.left
            h = next

        return nodes

    def get_height(self, root: Optional[TreeNode]) -> int:
        return 1 + self.get_height(root.left) if root is not None else -1


def main():
    inputs: list[Optional[TreeNode]] = [
        newnode(1, newnode(2, newval(4), newval(5)), newleft(3, newval(6))),
        None,
        newval(1),
        newnode(1, newleft(2, newval(4)), newval(3)),
    ]

    solution = Solution()
    for root in inputs:
        result = solution.countNodes(root)
        print(result)


if __name__ == "__main__":
    main()
