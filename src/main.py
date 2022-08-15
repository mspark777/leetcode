"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def convertToTitle(self, columnNumber: int) -> str:
        ACODE = ord('A')
        SIZE = 26
        n = columnNumber
        result: list[str] = []

        while n > 0:
            n -= 1

            temp = ACODE + (n % SIZE)
            result.append(chr(temp))

            n //= SIZE

        result.reverse()
        return "".join(result)



class Input:
    columnNumber: int

    def __init__(self, columnNumber: int):
        self.columnNumber = columnNumber


def main():
    inputs: list[Input] = [
        Input(1),
        Input(28),
        Input(701),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.convertToTitle(i.columnNumber)
        print(result)


if __name__ == "__main__":
    main()
