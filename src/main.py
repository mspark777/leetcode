from __future__ import annotations
from typing import List


class Solution:
    def maximalRectangle(self, matrix: List[List[str]]) -> int:
        col_count = len(matrix[0])
        heights = [0 for _ in range(col_count + 1)]
        result = 0

        for row in matrix:
            for col in range(col_count):
                if row[col] == "1":
                    heights[col] += 1
                else:
                    heights[col] = 0

            stack: list[int] = [-1]
            for col in range(col_count + 1):
                while heights[col] < heights[stack[-1]]:
                    h = heights[stack.pop()]
                    w = col - stack[-1] - 1
                    result = max(result, w * h)
                stack.append(col)

        return result


def main():
    inputs = [
        [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ],
        [["0"]],
        [["1"]],
        [["0", "1"], ["1", "0"]],
    ]

    for matrix in inputs:
        solution = Solution()
        result = solution.maximalRectangle(matrix)
        print(result)


if __name__ == "__main__":
    main()
