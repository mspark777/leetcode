from __future__ import annotations
from tree_node import TreeNode
from typing import Optional


class Solution:
    result: int
    last_val: Optional[int]

    def __init__(self):
        self.result = 10**5 + 1
        self.last_val = None

    def getMinimumDifference(self, root: Optional[TreeNode]) -> int:
        self.travel(root)
        return self.result

    def travel(self, node: Optional[TreeNode]):
        if node is None:
            return

        if node.left is not None:
            self.travel(node.left)

        if self.last_val is not None:
            self.result = min(self.result, abs(node.val - self.last_val))
            self.last_val = node.val
        else:
            self.last_val = node.val

        if node.right is not None:
            self.travel(node.right)


def main():
    input = (
        TreeNode(4, TreeNode(2, TreeNode(1), TreeNode(3)), TreeNode(6)),
        TreeNode(1, TreeNode(0), TreeNode(48, TreeNode(12), TreeNode(49))),
        TreeNode(
            236, TreeNode(104, None, TreeNode(227)), TreeNode(701, None, TreeNode(911))
        ),
    )

    for root in input:
        result = Solution().getMinimumDifference(root)
        print(result)


if __name__ == "__main__":
    main()
