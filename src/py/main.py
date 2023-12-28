from __future__ import annotations
from typing import List


class Solution:
    def islandPerimeter(self, grid: List[List[int]]) -> int:
        result = 0
        row_count = len(grid)
        col_count = len(grid[0])

        for row in range(row_count):
            for col in range(col_count):
                if grid[row][col] != 1:
                    continue

                result += 4
                if row > 0 and grid[row - 1][col] == 1:
                    result -= 1

                if col > 0 and grid[row][col - 1] == 1:
                    result -= 1

                if (row + 1) < row_count and grid[row + 1][col] == 1:
                    result -= 1

                if (col + 1) < col_count and grid[row][col + 1] == 1:
                    result -= 1

        return result


def main():
    input = ([[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]], [[1]], [[1, 0]])

    for grid in input:
        result = Solution().islandPerimeter(grid)
        print(result)


if __name__ == "__main__":
    main()
