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


def newnode(val: int, left: TreeNode, right: TreeNode) -> TreeNode:
    return TreeNode(val, left, right)


def newleft(val: int, left: TreeNode) -> TreeNode:
    return TreeNode(val, left, None)


def newright(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, None, right)


def newval(val: int) -> TreeNode:
    return TreeNode(val, None, None)


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if (p is None) and (q is None):
            return True

        if (p is None) or (q is None):
            return False

        if p.val != q.val:
            return False

        return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)


def main():
    inputs: list[tuple[TreeNode, TreeNode]] = [
        (newnode(1, newval(2), newval(3)), newnode(1, newval(2), newval(3))),
        (newleft(1, newval(2)), newright(1, newval(2))),
        (newnode(1, newval(2), newval(1)), newnode(1, newval(1), newval(2))),
    ]

    solution = Solution()
    for p, q in inputs:
        result = solution.isSameTree(p, q)
        print(result)


if __name__ == "__main__":
    main()
