"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def validUtf8(self, data: list[int]) -> bool:
        bytes = 0

        for i in data:
            if bytes == 0:
                mask = 128
                while (mask & i) != 0:
                    bytes += 1
                    mask >>= 1

                if bytes == 0:
                    continue

                if (bytes > 4) or (bytes == 1):
                    return False
            else:
                check0 = i & 128
                check1 = i & 64
                if (check0 == 0) or (check1 != 0):
                    return False

            bytes -= 1

        return bytes == 0


class Input:
    data: list[int]

    def __init__(self, data: list[int]):
        self.data = data


def main():
    inputs: list[Input] = [
        Input(data=[197, 130, 1]),
        Input(data=[235, 140, 4]),
    ]

    solution = Solution()
    for input in inputs:
        data = input.data
        result = solution.validUtf8(data)
        print(result)


if __name__ == "__main__":
    main()
