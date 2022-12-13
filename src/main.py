from __future__ import annotations
from typing import Optional, List


class Solution:
    def minFallingPathSum(self, matrix: List[List[int]]) -> int:
        row_count = len(matrix)
        col_count = len(matrix[0])
        last_row_idx = row_count - 1
        last_col_idx = col_count - 1

        dp = [[0 for j in range(col_count)] for i in range(row_count)]

        for i in range(col_count):
            dp[last_row_idx][i] = matrix[last_row_idx][i]

        for i in range(row_count - 2, -1, -1):
            for j in range(col_count):
                next = i + 1
                min_val = 2**31
                if j < last_col_idx:
                    min_val = min(dp[next][j + 1], min_val)

                if j > 0:
                    min_val = min(dp[next][j - 1], min_val)

                min_val = min(dp[next][j], min_val)
                dp[i][j] = matrix[i][j] + min_val

        min_val = 2**31
        for i in range(col_count):
            min_val = min(dp[0][i], min_val)

        return min_val


def main():
    inputs: list[list[list[int]]] = [
        [[2, 1, 3], [6, 5, 4], [7, 8, 9]],
        [[-19, 57], [-40, -5]],
    ]

    solution = Solution()
    for matrix in inputs:
        result = solution.minFallingPathSum(matrix)
        print(result)


if __name__ == "__main__":
    main()
