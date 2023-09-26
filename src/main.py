from __future__ import annotations
from collections import Counter
from typing import Optional, List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        cur = [[0] * 3 for _ in range(2)]
        next = [[0] * 3 for _ in range(2)]

        for idx in range(n - 1, -1, -1):
            for buy in range(2):
                for limits in range(1, 3):
                    profit = 0
                    if buy == 0:
                        buy_profit = prices[idx] + next[1][limits - 1]
                        skip_profit = next[0][limits]
                        profit = max(buy_profit, skip_profit)
                    else:
                        sell_profit = next[0][limits] - prices[idx]
                        skip_profit = next[1][limits]
                        profit = max(sell_profit, skip_profit)

                    cur[buy][limits] = profit
            next = cur.copy()

        return next[1][2]


def main():
    inputs = [[3, 3, 5, 0, 0, 3, 1, 4], [1, 2, 3, 4, 5], [7, 6, 4, 3, 1]]

    for prices in inputs:
        solution = Solution()
        result = solution.maxProfit(prices)
        print(result)


if __name__ == "__main__":
    main()
