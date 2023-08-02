from __future__ import annotations
from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        permutations: list[int] = []
        results: list[list[int]] = []
        self.backtrack(permutations, nums, results)

        return results

    def backtrack(
        self, permutations: list[int], nums: list[int], results: list[list[int]]
    ):
        if len(permutations) == len(nums):
            results.append(permutations.copy())
            return

        for num in nums:
            if num in permutations:
                continue

            permutations.append(num)
            self.backtrack(permutations, nums, results)
            permutations.pop()


def main():
    inputs = [[1, 2, 3], [0, 1], [1]]

    for nums in inputs:
        solution = Solution()
        result = solution.permute(nums)
        print(result)


if __name__ == "__main__":
    main()
