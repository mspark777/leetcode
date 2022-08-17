"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def titleToNumber(self, columnTitle: str) -> int:
        factor = ord("A") - 1
        result = 0

        for ch in columnTitle:
            code = ord(ch)
            result = result * 26 + (code - factor)

        return result


class Input:
    columnTitle: str

    def __init__(self, columnTitle: str):
        self.columnTitle = columnTitle


def main():
    inputs: list[Input] = [
        Input("A"),
        Input("AB"),
        Input("ZY"),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.titleToNumber(i.columnTitle)
        print(result)


if __name__ == "__main__":
    main()
