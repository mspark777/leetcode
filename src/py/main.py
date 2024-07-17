from __future__ import annotations
from typing import Optional, List
from tree_node import TreeNode, to_tree
from collections import deque


class Solution:
    def delNodes(
        self, root: Optional[TreeNode], to_delete: List[int]
    ) -> List[TreeNode]:
        if root is None:
            return []

        to_delete_set = set(to_delete)
        result: List[TreeNode] = []
        queue = deque([root])

        while queue:
            current = queue.popleft()
            if current.left is not None:
                queue.append(current.left)
                if current.left.val in to_delete_set:
                    current.left = None

            if current.right is not None:
                queue.append(current.right)
                if current.right.val in to_delete_set:
                    current.right = None

            if current.val in to_delete_set:
                if current.left is not None:
                    result.append(current.left)

                if current.right is not None:
                    result.append(current.right)

        if root.val not in to_delete_set:
            result.append(root)

        return result


def main():
    inputs: list[tuple[Optional[TreeNode], list[int]]] = [
        (to_tree([1, 2, 3, 4, 5, 6, 7]), [3, 5]),
        (to_tree([1, 2, 4, None, 3]), [3]),
    ]

    for root, to_delete in inputs:
        result = Solution().delNodes(root, to_delete)
        print(result)


if __name__ == "__main__":
    main()
