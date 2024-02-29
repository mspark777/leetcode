from __future__ import annotations
from typing import Optional
from tree_node import TreeNode, to_tree


class Solution:
    def isEvenOddTree(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return False

        nodes = [root]
        level = 0
        while nodes:
            node_count = len(nodes)
            is_even = level % 2 == 0
            prev = nodes[0].val + (-1 if is_even else 1)
            for i in range(node_count):
                node = nodes[i]
                val = node.val
                if not self.check_condition(is_even, val, prev):
                    return False
                prev = val

                if node.left is not None:
                    nodes.append(node.left)

                if node.right is not None:
                    nodes.append(node.right)

            level += 1
            nodes = nodes[node_count:]

        return True

    def check_condition(self, is_even: bool, val: int, prev: int) -> bool:
        if is_even:
            return val % 2 != 0 and val > prev

        return val % 2 == 0 and val < prev


def main():
    input: list[list[Optional[int]]] = [
        [1, 10, 4, 3, None, 7, 9, 12, 8, 6, None, None, 2],
        [5, 4, 2, 3, 3, 7],
        [5, 9, 1, 3, 5, 7],
    ]

    for vals in input:
        result = Solution().isEvenOddTree(to_tree(vals))
        print(result)


if __name__ == "__main__":
    main()
