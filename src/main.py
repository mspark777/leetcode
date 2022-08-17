"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def reverseBits(self, n: int) -> int:
        n = ((n & 0xFFFF0000) >> 16) | ((n & 0x0000FFFF) << 16)
        n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
        n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
        n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
        n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)

        return n


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(43261596),
        Input(4294967293),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.reverseBits(i.n)
        print(result)


if __name__ == "__main__":
    main()
