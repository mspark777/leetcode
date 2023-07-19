from __future__ import annotations
from typing import List


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        result: list[list[int]] = []
        self.solve(0, candidates, [], target, result)
        return result

    def solve(
        self,
        i: int,
        candidates: list[int],
        temp: list[int],
        target: int,
        result: list[list[int]],
    ):
        if target == 0:
            result.append(temp.copy())
            return

        if target < 0:
            return

        if i >= len(candidates):
            return

        self.solve(i + 1, candidates, temp, target, result)

        candidate = candidates[i]
        temp.append(candidate)
        self.solve(i, candidates, temp, target - candidate, result)
        temp.pop()


def main():
    inputs = [([2, 3, 6, 7], 7), ([2, 3, 5], 8), ([2], 1)]

    for candidates, target in inputs:
        solution = Solution()
        result = solution.combinationSum(candidates, target)
        print(result)


if __name__ == "__main__":
    main()
