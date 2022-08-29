"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible

WATER = "0"
LAND = "1"


class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        row_count = len(grid)
        col_count = len(grid[0])

        result = 0
        for r in range(row_count):
            for c in range(col_count):
                if grid[r][c] == LAND:
                    result += 1
                    self.clear_land(grid, r, c, row_count, col_count)

        return result

    def clear_land(
        self, grid: list[list[str]], row: int, col: int, row_count: int, col_count: int
    ):
        stack: list[tuple[int, int]] = [(row, col)]

        while len(stack) > 0:
            (r, c) = stack.pop()

            if (r < 0) or (c < 0):
                continue
            elif (r >= row_count) or (c >= col_count):
                continue
            elif grid[r][c] == WATER:
                continue

            grid[r][c] = WATER
            stack.append((r + 1, c))
            stack.append((r - 1, c))
            stack.append((r, c + 1))
            stack.append((r, c - 1))


class Input:
    grid: list[list[str]]

    def __init__(self, grid: list[list[str]]):
        self.grid = grid


def main():
    inputs: list[Input] = [
        Input(
            [
                ["1", "1", "1", "1", "0"],
                ["1", "1", "0", "1", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "0", "0", "0"],
            ]
        ),
        Input(
            [
                ["1", "1", "0", "0", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "1", "0", "0"],
                ["0", "0", "0", "1", "1"],
            ]
        ),
    ]

    solution = Solution()
    for input in inputs:
        grid = input.grid
        result = solution.numIslands(grid)
        print(result)


if __name__ == "__main__":
    main()
