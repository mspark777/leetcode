from __future__ import annotations
from typing import List


class Solution:
    def stoneGameII(self, piles: List[int]) -> int:
        n = len(piles)
        dp = [[0] * (n + 1) for _ in range(n + 1)]
        suffix_sum = [0] * (n + 1)
        for i in range(n - 1, -1, -1):
            suffix_sum[i] = suffix_sum[i + 1] + piles[i]
        for i in range(n + 1):
            dp[i][n] = suffix_sum[i]
        for index in range(n - 1, -1, -1):
            for max_till_now in range(n - 1, 0, -1):
                for X in range(1, min(2 * max_till_now, n - index) + 1):
                    dp[index][max_till_now] = max(
                        dp[index][max_till_now],
                        suffix_sum[index] - dp[index + X][max(max_till_now, X)],
                    )
        return dp[0][1]


def main():
    inputs: list[list[int]] = [[2, 7, 9, 4, 4], [1, 2, 3, 4, 5, 100]]

    for input in inputs:
        result = Solution().stoneGameII(input)
        print(result)


if __name__ == "__main__":
    main()
