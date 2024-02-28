from __future__ import annotations
from typing import Optional
from tree_node import TreeNode, to_tree
from collections import deque


class Solution:
    def findBottomLeftValue(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        queue = deque[TreeNode]()
        current = root
        queue.append(current)
        while queue:
            current = queue.popleft()
            if current.right is not None:
                queue.append(current.right)
            if current.left is not None:
                queue.append(current.left)

        return current.val


def main():
    input: list[list[Optional[int]]] = [
        [2, 1, 3],
        [1, 2, 3, 4, None, 5, 6, None, None, 7],
    ]

    for vals in input:
        result = Solution().findBottomLeftValue(to_tree(vals))
        print(result)


if __name__ == "__main__":
    main()
