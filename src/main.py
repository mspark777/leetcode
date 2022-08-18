"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        return n & (n - 1) == 0 if n > 0 else False


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(1),
        Input(16),
        Input(3),
    ]

    solution = Solution()
    for i in inputs:
        n = i.n
        result = solution.isPowerOfTwo(i.n)
        print(result)


if __name__ == "__main__":
    main()
