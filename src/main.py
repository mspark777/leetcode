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


def newright(val: int, right: Optional[TreeNode]) -> TreeNode:
    return newnode(val, None, right)


def newval(val: int) -> TreeNode:
    return newnode(val, None, None)


class Solution:
    def findTarget(self, root: Optional[TreeNode], k: int) -> bool:
        stack: list[Optional[TreeNode]] = [root]
        memo: set[int] = set()

        while stack:
            top = stack.pop()
            if top is None:
                continue

            val = top.val
            target = k - val
            if target in memo:
                return True

            memo.add(val)
            stack.append(top.left)
            stack.append(top.right)
        return False


def main():
    inputs: list[tuple[Optional[TreeNode], int]] = [
        (newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))), 9),
        (newnode(5, newnode(3, newval(2), newval(4)), newright(6, newval(7))), 28),
    ]

    solution = Solution()
    for root, k in inputs:
        result = solution.findTarget(root, k)
        print(result)


if __name__ == "__main__":
    main()
