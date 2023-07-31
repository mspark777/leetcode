from __future__ import annotations
from typing import List


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        results: list[list[int]] = []
        combinations: list[int] = []

        candidates.sort()
        self.backtrack(candidates, combinations, target, 0, results)
        return results

    def backtrack(
        self,
        candidates: list[int],
        combinations: list[int],
        remain: int,
        cur: int,
        results: list[list[int]],
    ):
        if remain == 0:
            results.append(combinations.copy())
            return

        for next in range(cur, len(candidates)):
            if (next > cur) and (candidates[next] == candidates[next - 1]):
                continue

            pick = candidates[next]
            next_remain = remain - pick
            if next_remain < 0:
                break

            combinations.append(pick)
            self.backtrack(candidates, combinations, next_remain, next + 1, results)
            combinations.pop()


def main():
    inputs = [([10, 1, 2, 7, 6, 1, 5], 8), ([2, 5, 2, 1, 2], 5)]

    for candidates, target in inputs:
        solution = Solution()
        result = solution.combinationSum2(candidates, target)
        print(result)


if __name__ == "__main__":
    main()
