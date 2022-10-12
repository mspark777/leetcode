from __future__ import annotations
from typing import Optional, List


class Solution:
    def largestPerimeter(self, nums: List[int]) -> int:
        nums.sort(reverse=True)

        for i in range(len(nums) - 2):
            a = nums[i]
            b = nums[i + 1] + nums[i + 2]
            if a < b:
                return a + b

        return 0


def main():
    inputs: list[list[int]] = [[2, 1, 2], [1, 2, 1]]

    solution = Solution()
    for nums in inputs:
        result = solution.largestPerimeter(nums)
        print(result)


if __name__ == "__main__":
    main()
