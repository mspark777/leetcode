from __future__ import annotations
from typing import Optional, List


class Solution:
    def minCost(self, colors: str, needed_time: List[int]) -> int:
        total = 0
        cur = needed_time[0]

        for i in range(1, len(colors)):
            if colors[i] != colors[i - 1]:
                cur = 0

            needed = needed_time[i]
            total += min(cur, needed)
            cur = max(cur, needed)

        return total


class Input:
    colors: str
    needed_time: list[int]

    def __init__(self, colors: str, needed_time: list[int]) -> None:
        self.colors = colors
        self.needed_time = needed_time


def main():
    inputs: list[Input] = [
        Input("abaac", [1, 2, 3, 4, 5]),
        Input("abc", [1, 2, 3]),
        Input("aabaa", [1, 2, 3, 4, 1]),
    ]

    solution = Solution()
    for input in inputs:
        colors = input.colors
        needed_time = input.needed_time
        result = solution.minCost(colors, needed_time)
        print(result)


if __name__ == "__main__":
    main()
