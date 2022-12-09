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
    return TreeNode(val, left)


def newright(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, None, right)


def newval(val: int) -> TreeNode:
    return TreeNode(val)


class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        return self.travel(root, root.val, root.val) if root is not None else 0

    def travel(self, node: Optional[TreeNode], curmax: int, curmin: int) -> int:
        if node is None:
            return curmax - curmin

        val = node.val
        left = node.left
        right = node.right
        curmax = max(curmax, val)
        curmin = min(curmin, val)

        l = self.travel(left, curmax, curmin)
        r = self.travel(right, curmax, curmin)

        return max(l, r)


def main():
    inputs: list[Optional[TreeNode]] = [
        newnode(
            8,
            newnode(3, newval(1), newnode(6, newval(4), newval(7))),
            newright(10, newleft(14, newval(13))),
        ),
        newright(1, newright(2, newleft(0, newval(3)))),
    ]

    solution = Solution()
    for root in inputs:
        result = solution.maxAncestorDiff(root)
        print(result)


if __name__ == "__main__":
    main()
