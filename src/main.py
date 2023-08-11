from __future__ import annotations
from typing import List


class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        dp = [0 for _ in range(amount + 1)]
        dp[0] = 1

        for i in range(len(coins) - 1, -1, -1):
            coin = coins[i]
            for j in range(coin, amount + 1):
                dp[j] += dp[j - coin]

        return dp[amount]


def main():
    inputs = [(5, [1, 2, 5]), (3, [2]), (10, [10])]

    for amount, coins in inputs:
        solution = Solution()
        result = solution.change(amount, coins)
        print(result)


if __name__ == "__main__":
    main()
