from __future__ import annotations
from typing import List


class Solution:
    def onesMinusZeros(self, grid: List[List[int]]) -> List[List[int]]:
        row_count = len(grid)
        col_count = len(grid[0])
        rows = [0 for _ in range(row_count)]
        cols = [0 for _ in range(col_count)]

        for r in range(row_count):
            for c in range(col_count):
                is_one = grid[r][c] == 1
                rows[r] += 1 if is_one else -1
                cols[c] += 1 if is_one else -1

        result = [[0 for _ in range(col_count)] for _ in range(row_count)]
        for r in range(row_count):
            for c in range(col_count):
                result[r][c] = rows[r] + cols[c]

        return result


def main():
    inputs = ([[0, 1, 1], [1, 0, 1], [0, 0, 1]], [[1, 1, 1], [1, 1, 1]])

    for mat in inputs:
        result = Solution().onesMinusZeros(mat)
        print(result)


if __name__ == "__main__":
    main()
