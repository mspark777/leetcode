"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible


class Solution:
    def rotate(self, matrix: list[list[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        self.transpose(matrix)
        self.reverse(matrix)

    def transpose(self, matrix: list[list[int]]) -> None:
        for i in range(len(matrix)):
            for j in range(i + 1, len(matrix)):
                temp = matrix[i][j]
                matrix[i][j] = matrix[j][i]
                matrix[j][i] = temp

    def reverse(self, matrix: list[list[int]]) -> None:
        for i in range(len(matrix)):
            j = 0
            k = len(matrix) - 1
            while j < k:
                temp = matrix[i][j]
                matrix[i][j] = matrix[i][k]
                matrix[i][k] = temp
                j += 1
                k -= 1


class Input:
    matrix: list[list[int]]

    def __init__(self, matrix: list[list[int]]):
        self.matrix = matrix


def main():
    inputs: list[Input] = [
        Input([[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
        Input([[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]]),
    ]

    solution = Solution()
    for input in inputs:
        matrix = input.matrix
        solution.rotate(matrix)
        print(matrix)


if __name__ == "__main__":
    main()
