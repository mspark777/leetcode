from __future__ import annotations
from typing import Optional, List


class Solution:
    def isToeplitzMatrix(self, matrix: List[List[int]]) -> bool:
        for r in range(1, len(matrix)):
            for c in range(1, len(matrix[r])):
                if matrix[r - 1][c - 1] != matrix[r][c]:
                    return False

        return True


def main():
    inputs: list[list[list[int]]] = [
        [[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]],
        [[1, 2], [2, 2]],
    ]

    solution = Solution()
    for matrix in inputs:
        result = solution.isToeplitzMatrix(matrix)
        print(result)


if __name__ == "__main__":
    main()
