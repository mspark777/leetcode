"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        if n <= 0:
            return False

        while (n % 3) == 0:
            n //= 3

        return n == 1


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(27),
        Input(0),
        Input(9),
        Input(45),
    ]

    solution = Solution()
    for input in inputs:
        n = input.n
        result = solution.isPowerOfThree(n)
        print(result)


if __name__ == "__main__":
    main()
