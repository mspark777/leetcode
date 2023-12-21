from __future__ import annotations
from typing import List


class Solution:
    def largestSubmatrix(self, matrix: List[List[int]]) -> int:
        row_count = len(matrix)
        col_count = len(matrix[0])
        prev_heights: list[tuple[int, int]] = []
        result = 0

        for row in range(row_count):
            heights: list[tuple[int, int]] = []
            seen: list[bool] = [False] * col_count

            for height, col in prev_heights:
                if matrix[row][col] == 1:
                    heights.append((height + 1, col))
                    seen[col] = True

            for col in range(col_count):
                if not seen[col] and matrix[row][col] == 1:
                    heights.append((1, col))

            for i, (height, _) in enumerate(heights):
                result = max(result, height * (i + 1))

            prev_heights = heights

        return result


def main():
    input = (
        [[0, 0, 1], [1, 1, 1], [1, 0, 1]],
        [[1, 0, 1, 0, 1]],
        [[1, 1, 0], [1, 0, 1]],
    )

    for matrix in input:
        result = Solution().largestSubmatrix(matrix)
        print(result)


if __name__ == "__main__":
    main()
