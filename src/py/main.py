from __future__ import annotations
from typing import List


class Solution:
    def buyChoco(self, prices: List[int], money: int) -> int:
        first = min(prices[0], prices[1])
        second = max(prices[0], prices[1])

        for price in prices[2:]:
            if first > price:
                second = first
                first = price
            elif second > price:
                second = price

        total = first + second
        if total > money:
            return money

        return money - total


def main():
    inputs = (([1, 2, 2], 3), ([3, 2, 3], 3))

    for prices, money in inputs:
        result = Solution().buyChoco(prices, money)
        print(result)


if __name__ == "__main__":
    main()
