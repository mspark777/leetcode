from __future__ import annotations
from typing import List


class Solution:
    def largestDivisibleSubset(self, nums: List[int]) -> List[int]:
        nums.sort()
        n = len(nums)
        prev: list[list[int]] = [[] for _ in range(n + 1)]
        for i in range(1, n + 1):
            curr: list[list[int]] = [[] for _ in range(n + 1)]
            for j in range(1, n + 1):
                x: list[int] = []
                if j == 1:
                    x = prev[i] + [nums[i - 1]]
                elif nums[j - 1] % nums[i - 1] == 0:
                    x = prev[i] + [nums[i - 1]]
                elif nums[i - 1] % nums[j - 1] == 0:
                    x = prev[i] + [nums[i - 1]]

                y = prev[j]
                if len(y) > len(x):
                    curr[j] = y[:]
                else:
                    curr[j] = x[:]
            prev = curr[:]

        return prev[1]


def main():
    inputs = ([1, 2, 3], [1, 2, 4, 8])

    for nums in inputs:
        result = Solution().largestDivisibleSubset(nums)
        print(result)


if __name__ == "__main__":
    main()
