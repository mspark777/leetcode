"""
main
"""

from __future__ import annotations
from typing import Optional


class Transaction:
    spend: int
    profit: int

    def __init__(self, spend: int, profit: int):
        self.spend = spend
        self.profit = profit


class Solution:
    def maxProfit(self, k: int, prices: list[int]) -> int:
        if k <= 0:
            return 0

        transactions = [Transaction(2**32, 0) for _ in range(k + 2)]

        for price in prices:
            for i in range(1, k + 2):
                prev = transactions[i - 1]
                cur = transactions[i]
                cur.spend = min(cur.spend, price - prev.profit)
                cur.profit = max(cur.profit, price - cur.spend)

        return transactions[k].profit


class Input:
    k: int
    prices: list[int]

    def __init__(self, k: int, prices: list[int]):
        self.k = k
        self.prices = prices


def main():
    inputs: list[Input] = [
        Input(2, [2, 4, 1]),
        Input(2, [3, 2, 6, 5, 0, 3]),
    ]

    solution = Solution()
    for input in inputs:
        k = input.k
        prices = input.prices
        result = solution.maxProfit(k, prices)
        print(result)


if __name__ == "__main__":
    main()
