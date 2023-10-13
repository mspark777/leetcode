from __future__ import annotations
from typing import Optional, List


class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        step0 = cost[0]
        step1 = cost[1]
        for step in cost[2:]:
            cur = step + min(step0, step1)
            step0 = step1
            step1 = cur

        return min(step0, step1)


def main():
    inputs = ([10, 15, 20], [1, 100, 1, 1, 1, 100, 1, 1, 100, 1])

    for cost in inputs:
        result = Solution().minCostClimbingStairs(cost)
        print(result)


if __name__ == "__main__":
    main()
