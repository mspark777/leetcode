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
    def levelOrderBottom(self, root: Optional[TreeNode]) -> List[List[int]]:
        result: list[list[int]] = []
        if root is None:
            return result

        queue: list[TreeNode] = [root]
        while queue:
            node_count = len(queue)
            current: list[int] = []
            for i in range(node_count):
                node = queue[i]
                current.append(node.val)
                if node.left is not None:
                    queue.append(node.left)

                if node.right is not None:
                    queue.append(node.right)

            result.append(current)
            queue = queue[node_count:]

        result.reverse()
        return result


def main():
    inputs = [
        TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7))),
        TreeNode(1),
        None,
    ]

    for root in inputs:
        solution = Solution()
        result = solution.levelOrderBottom(root)
        print(result)


if __name__ == "__main__":
    main()
