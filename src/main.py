from __future__ import annotations
from typing import Optional, List


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


class Dfs:
    result: int

    def __init__(self):
        self.result = -2147483647

    def dfs(self, node: Optional[TreeNode]) -> int:
        if node is None:
            return 0

        left = max(0, self.dfs(node.left))
        right = max(0, self.dfs(node.right))

        path0 = node.val + left + right
        path1 = node.val + max(left, right)
        self.result = max(self.result, path0, path1)
        return path1


class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        dfs = Dfs()
        dfs.dfs(root)
        return dfs.result


def main():
    inputs = (
        TreeNode(1, TreeNode(2), TreeNode(3)),
        TreeNode(-10, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7))),
        TreeNode(2, TreeNode(-1), TreeNode(-2)),
    )

    for root in inputs:
        result = Solution().maxPathSum(root)
        print(result)


if __name__ == "__main__":
    main()
