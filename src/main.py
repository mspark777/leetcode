from __future__ import annotations
from typing import List
from functools import reduce


class Solution:
    def maxRunTime(self, n: int, batteries: List[int]) -> int:
        left = 1
        right = sum(batteries) // n

        while left < right:
            target = (left + right + 1) // 2
            extra = reduce(lambda acc, power: acc + min(power, target), batteries, 0)
            if extra >= (n * target):
                left = target
            else:
                right = target - 1

        return left


def main():
    inputs = [(2, [3, 3, 3]), (2, [1, 1, 1, 1])]

    for n, batteries in inputs:
        solution = Solution()
        result = solution.maxRunTime(n, batteries)
        print(result)


if __name__ == "__main__":
    main()
