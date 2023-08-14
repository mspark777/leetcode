from __future__ import annotations
from typing import List
from enum import Enum


class Direction(Enum):
    LEFT = 0
    RIGHT = 1
    UP = 2
    DOWN = 3


class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        row_count = len(matrix)
        col_count = len(matrix[0])
        left = 0
        right = col_count - 1
        top = 0
        bottom = row_count - 1
        dir = Direction.RIGHT
        result: list[int] = []

        while (left <= right) and (top <= bottom):
            if dir == Direction.RIGHT:
                for col in range(left, right + 1):
                    result.append(matrix[top][col])
                top += 1
                dir = Direction.DOWN
            elif dir == Direction.DOWN:
                for row in range(top, bottom + 1):
                    result.append(matrix[row][right])
                right -= 1
                dir = Direction.LEFT
            elif dir == Direction.LEFT:
                for col in range(right, left - 1, -1):
                    result.append(matrix[bottom][col])
                bottom -= 1
                dir = Direction.UP
            else:
                for row in range(bottom, top - 1, -1):
                    result.append(matrix[row][left])
                left += 1
                dir = Direction.RIGHT

        return result


def main():
    inputs = [
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
    ]

    for matrix in inputs:
        solution = Solution()
        result = solution.spiralOrder(matrix)
        print(result)


if __name__ == "__main__":
    main()
