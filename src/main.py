from __future__ import annotations
from typing import List


class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        results: list[list[int]] = []
        combinations: list[int] = []
        self.backtrack(1, combinations, results, n, k)

        return results

    def backtrack(
        self,
        start: int,
        combinations: list[int],
        results: list[list[int]],
        n: int,
        k: int,
    ):
        if len(combinations) == k:
            results.append(combinations.copy())
            return

        for i in range(start, n + 1):
            combinations.append(i)
            self.backtrack(i + 1, combinations, results, n, k)
            combinations.pop()


def main():
    inputs = [(4, 2), (1, 1)]

    for n, k in inputs:
        solution = Solution()
        result = solution.combine(n, k)
        print(result)


if __name__ == "__main__":
    main()
