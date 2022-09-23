"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def concatenatedBinary(self, n: int) -> int:
        result = 1
        len = 4
        mod = 1000000007

        for i in range(2, n + 1):
            if i == len:
                len *= 2

            result = ((result * len) + i) % mod

        return result


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(1),
        Input(3),
        Input(12),
    ]

    solution = Solution()
    for input in inputs:
        n = input.n
        result = solution.concatenatedBinary(n)
        print(result)


if __name__ == "__main__":
    main()
