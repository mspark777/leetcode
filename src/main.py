from __future__ import annotations
from typing import Optional


class Solution:
    def climbStairs(self, n: int) -> int:
        n0 = 1
        n1 = 1

        for i in range(1, n):
            sum = n0 + n1
            n0 = n1
            n1 = sum

        return n1


def main():
    inputs: list[int] = [2, 3]

    solution = Solution()
    for n in inputs:
        result = solution.climbStairs(n)
        print(result)


if __name__ == "__main__":
    main()
