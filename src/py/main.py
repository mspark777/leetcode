from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def rangeSumBST(self, root: Optional[TreeNode], low: int, high: int) -> int:
        if root is None:
            return 0

        val = root.val
        result = 0
        if low <= val <= high:
            result += val

        result += self.rangeSumBST(root.left, low, high)
        result += self.rangeSumBST(root.right, low, high)

        return result


def main():
    input = (
        (
            TreeNode(
                10,
                TreeNode(5, TreeNode(3), TreeNode(7)),
                TreeNode(15, None, TreeNode(18)),
            ),
            7,
            15,
        ),
        (
            TreeNode(
                10,
                TreeNode(
                    5,
                    TreeNode(3, TreeNode(1)),
                    TreeNode(7, TreeNode(6)),
                ),
                TreeNode(15, TreeNode(13), TreeNode(18)),
            ),
            6,
            10,
        ),
    )

    for root, low, high in input:
        result = Solution().rangeSumBST(root, low, high)
        print(result)


if __name__ == "__main__":
    main()
