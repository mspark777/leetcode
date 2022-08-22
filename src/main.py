"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isPowerOfFour(self, n: int) -> bool:
        return (n > 0) and ((n & (n - 1)) == 0) and ((n & 0x55555555) != 0)


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [Input(16), Input(5), Input(1)]

    solution = Solution()
    for i in inputs:
        n = i.n
        result = solution.isPowerOfFour(n)
        print(result)


if __name__ == "__main__":
    main()
