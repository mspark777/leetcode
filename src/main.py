from __future__ import annotations
from typing import List


class Solution:
    def PredictTheWinner(self, nums: List[int]) -> bool:
        n = len(nums)
        dp = nums.copy()

        for diff in range(1, n):
            for left in range(n - diff):
                right = left + diff
                lnum = nums[left]
                rnum = nums[right]
                ldp = dp[left]
                rdp = dp[left + 1]
                dp[left] = max(lnum - rdp, rnum - ldp)

        return dp[0] >= 0


def main():
    inputs = [[1, 5, 2], [1, 5, 233, 7]]

    for nums in inputs:
        solution = Solution()
        result = solution.PredictTheWinner(nums)
        print(result)


if __name__ == "__main__":
    main()
