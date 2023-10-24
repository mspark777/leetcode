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


class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        result: list[int] = []
        stack: list[tuple[TreeNode, int]] = [(root, 0)]

        while stack:
            node, depth = stack.pop()
            if depth == len(result):
                result.append(node.val)
            else:
                result[depth] = max(result[depth], node.val)

            if node.left is not None:
                stack.append((node.left, depth + 1))
            if node.right is not None:
                stack.append((node.right, depth + 1))

        return result


def main():
    inputs = (
        TreeNode(
            1, TreeNode(3, TreeNode(5), TreeNode(3)), TreeNode(2, right=TreeNode(9))
        ),
        TreeNode(1, TreeNode(2), TreeNode(3)),
    )

    for root in inputs:
        result = Solution().largestValues(root)
        print(result)


if __name__ == "__main__":
    main()
