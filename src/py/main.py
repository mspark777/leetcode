from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    result: int

    def __init__(self):
        self.result = 0

    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.resolve(root)
        return self.result

    def resolve(self, node: Optional[TreeNode]) -> int:
        if node is None:
            return 0

        ldepth = self.resolve(node.left)
        rdepth = self.resolve(node.right)
        self.result = max(self.result, ldepth + rdepth)
        return 1 + max(ldepth, rdepth)


def main():
    input = (TreeNode(1, TreeNode(2)), TreeNode(1, TreeNode(2), TreeNode(3)))

    for root in input:
        result = Solution().diameterOfBinaryTree(root)
        print(result)


if __name__ == "__main__":
    main()
