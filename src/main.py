"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible


class Solution:
    def addDigits(self, num: int) -> int:
        return 1 + (num - 1) % 9 if num != 0 else 0


class Input:
    num: int

    def __init__(self, num: int):
        self.num = num


def main():
    inputs: list[Input] = [
        Input(38),
        Input(0),
    ]

    solution = Solution()
    for input in inputs:
        num = input.num
        result = solution.addDigits(num)
        print(result)


if __name__ == "__main__":
    main()
