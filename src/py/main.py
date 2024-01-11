from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


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
    input = (
        TreeNode(
            8,
            TreeNode(3, TreeNode(1), TreeNode(6, TreeNode(4), TreeNode(7))),
            TreeNode(10, None, TreeNode(14, TreeNode(13))),
        ),
        TreeNode(1, None, TreeNode(2, None, TreeNode(0, TreeNode(3)))),
    )

    for root in input:
        result = Solution().maxAncestorDiff(root)
        print(result)


if __name__ == "__main__":
    main()
