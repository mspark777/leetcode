from typing import List


class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        row = len(grid)
        col = len(grid[0])
        last_row = row - 1
        last_col = col - 1
        MAX = 2**31

        for r in range(last_row, -1, -1):
            for c in range(last_col, -1, -1):
                if (r == last_row) and (c == last_col):
                    continue

                right_min = grid[r][c + 1] if c < last_col else MAX
                down_min = grid[r + 1][c] if r < last_row else MAX
                grid[r][c] += min(right_min, down_min)

        return grid[0][0]


def main():
    inputs: list[list[list[int]]] = [
        [[1, 3, 1], [1, 5, 1], [4, 2, 1]],
        [[1, 2, 3], [4, 5, 6]],
    ]

    for grid in inputs:
        solution = Solution()
        result = solution.minPathSum(grid)
        print(result)


if __name__ == "__main__":
    main()
