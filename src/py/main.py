from __future__ import annotations
from collections import defaultdict
from typing import Optional, List
from tree_node import TreeNode


class Solution:
    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        counter = defaultdict[int, int](int)
        stack = [root]

        while stack:
            node = stack.pop()
            counter[node.val] += 1

            left = node.left
            if left is not None:
                stack.append(left)

            right = node.right
            if right is not None:
                stack.append(right)

        max_freq = max(counter.values())

        result: list[int] = []
        for val, freq in counter.items():
            if freq == max_freq:
                result.append(val)

        return result


def main():
    inputs = (TreeNode(1, None, TreeNode(2, TreeNode(2))), TreeNode(0))

    for root in inputs:
        result = Solution().findMode(root)
        print(result)


if __name__ == "__main__":
    main()
