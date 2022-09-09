"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def numberOfWeakCharacters(self, properties: list[list[int]]) -> int:
        properties.sort(key=lambda p: (-p[0], p[1]))

        result = 0
        max_defence = 0
        for _attack, defence in properties:
            if max_defence > defence:
                result += 1
            else:
                max_defence = defence

        return result


class Input:
    properties: list[list[int]]

    def __init__(self, properties: list[list[int]]):
        self.properties = properties


def main():
    inputs: list[Input] = [
        Input([[5, 5], [6, 3], [3, 6]]),
        Input([[2, 2], [3, 3]]),
        Input([[1, 5], [10, 4], [4, 3]]),
    ]

    solution = Solution()
    for input in inputs:
        properties = input.properties
        result = solution.numberOfWeakCharacters(properties)
        print(result)


if __name__ == "__main__":
    main()
