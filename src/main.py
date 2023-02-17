from __future__ import annotations
from typing import Optional


class TreeNode:
    val: int
    left: Optional[TreeNode]
    right: Optional[TreeNode]

    def __init__(
        self,
        val: int = 0,
        left: Optional[TreeNode] = None,
        right: Optional[TreeNode] = None,
    ):
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
    return TreeNode(val)


class Solution:
    def minDiffInBST(self, root: Optional[TreeNode]) -> int:
        return self.travel(root, [None], [2**31])

    def travel(
        self,
        root: Optional[TreeNode],
        prev: list[Optional[TreeNode]],
        distance: list[int],
    ) -> int:
        if root is None:
            return distance[0]

        self.travel(root.left, prev, distance)

        if prev[0] is not None:
            distance[0] = min(distance[0], root.val - prev[0].val)
        prev[0] = root
        self.travel(root.right, prev, distance)
        return distance[0]


def main():
    inputs: list[Optional[TreeNode]] = [
        newnode(4, newnode(2, newval(1), newval(3)), newval(6)),
        newnode(1, newval(0), newnode(48, newval(12), newval(49))),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.minDiffInBST(root)
        print(result)


if __name__ == "__main__":
    main()
