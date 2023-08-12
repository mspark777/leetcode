from __future__ import annotations
from typing import List


class Solution:
    def uniquePathsWithObstacles(self, obstacle_grid: List[List[int]]) -> int:
        OBSTACLE = 1
        row_count = len(obstacle_grid)
        col_count = len(obstacle_grid[0])
        count_grid = [[0 for _ in range(col_count)] for _ in range(row_count)]
        for r in range(row_count):
            for c in range(col_count):
                if obstacle_grid[r][c] == OBSTACLE:
                    continue

                if (r + c) == 0:
                    count_grid[r][c] = 1
                elif r == 0:
                    count_grid[r][c] = count_grid[r][c - 1]
                elif c == 0:
                    count_grid[r][c] = count_grid[r - 1][c]
                else:
                    count_grid[r][c] = count_grid[r][c - 1] + count_grid[r - 1][c]

        return count_grid[row_count - 1][col_count - 1]


def main():
    inputs = [[[0, 0, 0], [0, 1, 0], [0, 0, 0]], [[0, 1], [0, 0]]]

    for grid in inputs:
        solution = Solution()
        result = solution.uniquePathsWithObstacles(grid)
        print(result)


if __name__ == "__main__":
    main()
