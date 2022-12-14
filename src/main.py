from __future__ import annotations
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        robbed = 0
        no_robbed = 0

        for n in nums:
            temp = no_robbed
            no_robbed = max(no_robbed, robbed)
            robbed = n + temp

        return max(no_robbed, robbed)


def main():
    inputs: list[list[int]] = [[1, 2, 3, 1], [2, 7, 9, 3, 1]]

    solution = Solution()
    for nums in inputs:
        result = solution.rob(nums)
        print(result)


if __name__ == "__main__":
    main()
