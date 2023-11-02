from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def averageOfSubtree(self, root: Optional[TreeNode]) -> int:
        result = self.travel(root)
        return result[1]

    def travel(self, node: Optional[TreeNode]) -> tuple[int, int, int]:
        if node is None:
            return (0, 0, 0)

        (lsum, lres, lcnt) = self.travel(node.left)
        (rsum, rres, rcnt) = self.travel(node.right)

        sum = lsum + rsum + node.val
        cnt = lcnt + rcnt + 1
        avg = sum // cnt
        if node.val == avg:
            return (sum, lres + rres + 1, cnt)
        return (sum, lres + rres, cnt)


def main():
    inputs = (
        TreeNode(
            4, TreeNode(8, TreeNode(0), TreeNode(1)), TreeNode(5, None, TreeNode(6))
        ),
        TreeNode(1),
    )

    for root in inputs:
        result = Solution().averageOfSubtree(root)
        print(result)


if __name__ == "__main__":
    main()
