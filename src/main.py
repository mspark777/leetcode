"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isUgly(self, n: int) -> bool:
        while n > 1:
            if (n % 2) == 0:
                n //= 2
            elif (n % 3) == 0:
                n //= 3
            elif (n % 5) == 0:
                n //= 5
            else:
                return False

        return n == 1


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(6),
        Input(1),
        Input(14),
        Input(-2147483648),
    ]

    solution = Solution()
    for input in inputs:
        n = input.n
        result = solution.isUgly(n)
        print(result)


if __name__ == "__main__":
    main()
