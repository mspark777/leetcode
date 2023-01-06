from __future__ import annotations
from typing import List


class Solution:
    def maxIceCream(self, costs: List[int], coins: int) -> int:
        costs.sort()

        result = 0

        for cost in costs:
            if coins >= cost:
                coins -= cost
                result += 1
            else:
                break

        return result


def main():
    inputs: list[tuple[list[int], int]] = [
        ([1, 3, 2, 4, 1], 7),
        ([10, 6, 8, 7, 7, 8], 5),
        ([1, 6, 3, 1, 2, 5], 20),
    ]

    solution = Solution()
    for costs, coins in inputs:
        result = solution.maxIceCream(costs, coins)
        print(result)


if __name__ == "__main__":
    main()
