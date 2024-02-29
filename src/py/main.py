from __future__ import annotations
from typing import Optional
from tree_node import TreeNode, to_tree


class Solution:
    def isSubtree(self, root: Optional[TreeNode], sub_root: Optional[TreeNode]) -> bool:
        if root is None:
            return False
        if self.dfs(root, sub_root):
            return True

        return self.isSubtree(root.left, sub_root) or self.isSubtree(
            root.right, sub_root
        )

    def dfs(
        self,
        node: Optional[TreeNode],
        sub: Optional[TreeNode],
    ) -> bool:
        if node is None and sub is None:
            return True
        elif node is None or sub is None:
            return False
        elif node.val != sub.val:
            return False

        return self.dfs(node.left, sub.left) and self.dfs(node.right, sub.right)


def main():
    input = [
        ([[3, 4, 5, 1, 2], [4, 1, 2]]),
        ([3, 4, 5, 1, 2, None, None, None, None, 0], [4, 1, 2]),
        ([1, 1], [1]),
    ]
    for root, sub_root in input:
        result = Solution().isSubtree(to_tree(root), to_tree(sub_root))
        print(result)


if __name__ == "__main__":
    main()
