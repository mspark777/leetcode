from __future__ import annotations
from typing import Optional
from tree_node import TreeNode


class Solution:
    def tree2str(self, root: Optional[TreeNode]) -> str:
        if root is None:
            return ""

        left = self.tree2str(root.left)
        right = self.tree2str(root.right)
        val = str(root.val)

        if left == "":
            if right == "":
                return val
            else:
                return f"{val}()({right})"
        elif right == "":
            return f"{val}({left})"

        return f"{val}({left})({right})"


def main():
    inputs = (
        TreeNode(1, TreeNode(2, TreeNode(4)), TreeNode(3)),
        TreeNode(1, TreeNode(2, None, TreeNode(4)), TreeNode(3)),
    )

    for root in inputs:
        result = Solution().tree2str(root)
        print(result)


if __name__ == "__main__":
    main()
