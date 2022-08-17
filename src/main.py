"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isHappy(self, n: int) -> bool:
        slow = n
        fast = self.next(n)

        while (fast != 1) and (slow != fast):
            slow = self.next(slow)
            fast = self.next(self.next(fast))

        return fast == 1

    def next(self, n: int) -> int:
        result = 0
        while n > 0:
            d = n % 10
            result += d * d
            n //= 10

        return result


class Input:
    n: int

    def __init__(self, n: int):
        self.n = n


def main():
    inputs: list[Input] = [
        Input(19),
        Input(2),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.isHappy(i.n)
        print(result)


if __name__ == "__main__":
    main()
