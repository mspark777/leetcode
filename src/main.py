"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def reorderedPowerOf2(self, n: int) -> bool:
        counts = self.get_counts(n)
        for i in range(32):
            if counts == self.get_counts(1 << i):
                return True
        return False

    def get_counts(self, n: int) -> list[int]:
        result = [0] * 10
        while n > 0:
            result[n % 10] += 1
            n //= 10
        return result


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [Input(1), Input(10), Input(46)]

    solution = Solution()
    for input in inputs:
        n = input.n
        result = solution.reorderedPowerOf2(n)
        print(result)


if __name__ == "__main__":
    main()
