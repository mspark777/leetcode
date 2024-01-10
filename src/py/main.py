from __future__ import annotations
from typing import Optional
from tree_node import (
    TreeNode,
    new_tree_val,
    new_tree_node,
    new_tree_right,
)


class Solution:
    max_distance: int

    def __init__(self):
        self.max_distance = 0

    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        self.travel(root, start)
        return self.max_distance

    def travel(self, node: Optional[TreeNode], start: int) -> int:
        if node is None:
            return 0

        depth = 0
        left_depth = self.travel(node.left, start)
        right_depth = self.travel(node.right, start)
        if node.val == start:
            self.max_distance = max(left_depth, right_depth)
            depth = -1
        elif left_depth >= 0 and right_depth >= 0:
            depth = max(left_depth, right_depth) + 1
        else:
            distance = abs(left_depth) + abs(right_depth)
            self.max_distance = max(self.max_distance, distance)
            depth = min(left_depth, right_depth) - 1

        return depth


def main():
    input = (
        (
            new_tree_node(
                1,
                new_tree_right(5, new_tree_node(4, new_tree_val(9), new_tree_val(2))),
                new_tree_node(3, new_tree_val(10), new_tree_val(6)),
            ),
            3,
        ),
        (new_tree_val(1), 1),
    )

    for root, start in input:
        result = Solution().amountOfTime(root, start)
        print(result)


if __name__ == "__main__":
    main()
