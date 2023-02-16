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


def newright(val: int, right: TreeNode) -> TreeNode:
    return TreeNode(val, None, right)


def newval(val: int) -> TreeNode:
    return TreeNode(val)


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        result = [0]
        self.travel(root, 0, result)

        return result[0]

    def travel(self, node: Optional[TreeNode], depth: int, ref: list[int]):
        if node:
            d = depth + 1
            self.travel(node.left, d, ref)
            self.travel(node.right, d, ref)
        else:
            ref[0] = max(ref[0], depth)


def main():
    inputs: list[Optional[TreeNode]] = [
        newnode(3, newval(9), newnode(20, newval(15), newval(7))),
        newright(1, newval(2)),
    ]

    for root in inputs:
        solution = Solution()
        result = solution.maxDepth(root)
        print(result)


if __name__ == "__main__":
    main()
