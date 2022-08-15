"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def romanToInt(self, s: str) -> int:
        result = 0
        num = 0

        for ch in reversed(s):
            if ch == "I":
                num = 1
            elif ch == "V":
                num = 5
            elif ch == "X":
                num = 10
            elif ch == "L":
                num = 50
            elif ch == "C":
                num = 100
            elif ch == "D":
                num = 500
            elif ch == "M":
                num = 1000

            temp = num * 4
            if temp < result:
                result -= num
            else:
                result += num

        return result


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs: list[Input] = [
        Input("III"),
        Input("LVIII"),
        Input("MCMXCIV"),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.romanToInt(i.s)
        print(result)


if __name__ == "__main__":
    main()
