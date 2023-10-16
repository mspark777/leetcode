from __future__ import annotations
from typing import Optional, List


class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        n = len(gas)
        total = 0
        surplus = 0
        start = 0

        for i in range(n):
            diff = gas[i] - cost[i]
            total += diff
            surplus += diff

            if surplus < 0:
                surplus = 0
                start = i + 1

        return start if total >= 0 else -1


def main():
    inputs = (([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]), ([2, 3, 4], [3, 4, 3]))

    for gas, cost in inputs:
        result = Solution().canCompleteCircuit(gas, cost)
        print(result)


if __name__ == "__main__":
    main()
