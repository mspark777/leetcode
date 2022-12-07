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
    def rangeSumBST(self, root: Optional[TreeNode], low: int, high: int) -> int:
        result = 0

        stack = [root]
        while stack:
            top = stack.pop()
            if top is None:
                continue

            val = top.val
            left = top.left
            right = top.right

            if (low <= val) and (val <= high):
                result += val

            if low < val:
                stack.append(left)

            if val < high:
                stack.append(right)

        return result


def main():
    inputs: list[tuple[int, int, Optional[TreeNode]]] = [
        (
            7,
            15,
            newnode(10, newnode(5, newval(3), newval(7)), newright(15, newval(18))),
        ),
        (
            6,
            10,
            newnode(
                10,
                newnode(5, newleft(3, newval(1)), newleft(7, newval(6))),
                newnode(5, newval(13), newval(18)),
            ),
        ),
    ]

    solution = Solution()
    for low, high, root in inputs:
        result = solution.rangeSumBST(root, low, high)
        print(result)


if __name__ == "__main__":
    main()
