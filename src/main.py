from typing import List


class Solution:
    def countNegatives(self, grid: List[List[int]]) -> int:
        result = 0
        col_count = len(grid[0])
        cur = col_count - 1

        for row in grid:
            while cur >= 0:
                if row[cur] < 0:
                    cur -= 1
                else:
                    break

            result += col_count - (cur + 1)

        return result


def main():
    inputs = [
        [[4, 3, 2, -1], [3, 2, 1, -1], [1, 1, -1, -2], [-1, -1, -2, -3]],
        [[3, 2], [1, 0]],
    ]

    for grid in inputs:
        solution = Solution()
        result = solution.countNegatives(grid)
        print(result)


if __name__ == "__main__":
    main()
