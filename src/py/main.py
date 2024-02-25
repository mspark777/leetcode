from __future__ import annotations
from tree_node import TreeNode
from typing import Optional


class Solution:
    result: int

    def __init__(self):
        self.result = 0

    def findTilt(self, root: Optional[TreeNode]) -> int:
        self.postorder(root)
        return self.result

    def postorder(self, node: Optional[TreeNode]) -> int:
        if node is None:
            return 0

        left_total = self.postorder(node.left)
        right_total = self.postorder(node.right)
        self.result += abs(left_total - right_total)

        return node.val + left_total + right_total


def main():
    input = (
        TreeNode(1, TreeNode(2), TreeNode(3)),
        TreeNode(
            4, TreeNode(2, TreeNode(3), TreeNode(5)), TreeNode(9, None, TreeNode(7))
        ),
        TreeNode(
            21,
            TreeNode(7, TreeNode(1, TreeNode(3), TreeNode(3)), TreeNode(1)),
            TreeNode(14, TreeNode(2), TreeNode(2)),
        ),
    )

    for root in input:
        result = Solution().findTilt(root)
        print(result)


if __name__ == "__main__":
    main()
