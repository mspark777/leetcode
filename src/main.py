from __future__ import annotations
from typing import Optional
from collections import deque


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
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> list[list[int]]:
        if root is None:
            return []

        result: list[list[int]] = []
        node_queue = deque[tuple[TreeNode, int]]([(root, 0)])
        prev_level: Optional[int] = None
        level_nodes = deque[int]()

        while node_queue:
            node, level = node_queue.popleft()
            if prev_level is not None:
                if prev_level != level:
                    result.append(list(level_nodes))
                    level_nodes = deque()

            if (level & 1) == 0:
                level_nodes.append(node.val)
            else:
                level_nodes.appendleft(node.val)
            prev_level = level

            if node.left is not None:
                node_queue.append((node.left, level + 1))

            if node.right is not None:
                node_queue.append((node.right, level + 1))

        if level_nodes:
            result.append(list(level_nodes))

        return result


def main():
    inputs = [
        TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7))),
        TreeNode(1),
        None,
    ]

    for root in inputs:
        solution = Solution()
        result = solution.zigzagLevelOrder(root)
        print(result)


if __name__ == "__main__":
    main()
