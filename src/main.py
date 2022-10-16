from __future__ import annotations
from typing import Optional, List


class Solution:
    def minDifficulty(self, jobDifficulty: List[int], d: int) -> int:
        days = len(jobDifficulty)
        if days < d:
            return -1

        dp = [0 for i in range(days + 1)]
        for i in range(days - 1, -1, -1):
            dp[i] = max(dp[i + 1], jobDifficulty[i])

        for i in range(2, d + 1):
            remain = days - i
            for j in range(0, remain + 1):
                maxd = 0
                dp[j] = 2147483647
                for k in range(j, remain + 1):
                    maxd = max(maxd, jobDifficulty[k])
                    dp[j] = min(dp[j], maxd + dp[k + 1])

        return dp[0]


def main():
    inputs: list[tuple[list[int], int]] = [
        ([6, 5, 4, 3, 2, 1], 2),
        ([9, 9, 9], 4),
        ([1, 1, 1], 3),
    ]

    solution = Solution()
    for jobDifficulty, d in inputs:
        result = solution.minDifficulty(jobDifficulty, d)
        print(result)


if __name__ == "__main__":
    main()
