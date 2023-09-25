from __future__ import annotations
from typing import Optional, List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        result = 0

        for i in range(len(prices) - 1):
            cur = prices[i]
            next = prices[i + 1]
            if cur < next:
                result += next - cur

        return result


def main():
    inputs = [[7, 1, 5, 3, 6, 4], [1, 2, 3, 4, 5], [7, 6, 4, 3, 1]]

    for prices in inputs:
        solution = Solution()
        result = solution.maxProfit(prices)
        print(result)


if __name__ == "__main__":
    main()
