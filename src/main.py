from __future__ import annotations
from typing import List


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result: list[list[int]] = []
        self.solve(0, nums, [], result)

        return result

    def solve(self, i: int, nums: list[int], temp: list[int], result: list[list[int]]):
        result.append(temp.copy())

        for j in range(i, len(nums)):
            temp.append(nums[j])
            self.solve(j + 1, nums, temp, result)
            temp.pop()


def main():
    inputs = [[1, 2, 3], [0]]

    for nums in inputs:
        solution = Solution()
        result = solution.subsets(nums)
        print(result)


if __name__ == "__main__":
    main()
