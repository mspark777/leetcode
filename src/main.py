from __future__ import annotations
from typing import List


class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        result = [0 for _ in range(target + 1)]
        result[0] = 1
        for i in range(target + 1):
            for num in nums:
                if i >= num:
                    result[i] += result[i - num]

        return result[-1]


def main():
    inputs = [([1, 2, 3], 4), ([9], 3)]

    for nums, target in inputs:
        solution = Solution()
        result = solution.combinationSum4(nums, target)
        print(result)


if __name__ == "__main__":
    main()
