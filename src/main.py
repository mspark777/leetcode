from __future__ import annotations
from typing import Optional, List


class Solution:
    def validateBinaryTreeNodes(
        self, n: int, left: List[int], right: List[int]
    ) -> bool:
        root = self.find_root(n, left, right)
        if root == -1:
            return False

        seen = set[int]([root])
        stack: list[int] = [root]

        while stack:
            node = stack.pop()
            for child in (left[node], right[node]):
                if child == -1:
                    continue

                if child in seen:
                    return False

                stack.append(child)
                seen.add(child)

        return len(seen) == n

    def find_root(self, n: int, left: list[int], right: list[int]) -> int:
        children = set(left) | set(right)

        for i in range(n):
            if i not in children:
                return i

        return -1


def main():
    inputs = (
        (4, [1, -1, 3, -1], [2, -1, -1, -1]),
        (4, [1, -1, 3, -1], [2, 3, -1, -1]),
        (2, [1, 0], [-1, -1]),
    )

    for n, left, right in inputs:
        result = Solution().validateBinaryTreeNodes(n, left, right)
        print(result)


if __name__ == "__main__":
    main()
