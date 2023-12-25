from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        s = 0
        if root.left is not None:
            left = root.left
            if left.left is None and left.right is None:
                s += left.val

        s += self.sumOfLeftLeaves(root.left)
        s += self.sumOfLeftLeaves(root.right)

        return s


def main():
    input = (
        TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7))),
        TreeNode(1),
    )

    for root in input:
        result = Solution().sumOfLeftLeaves(root)
        print(result)


if __name__ == "__main__":
    main()
