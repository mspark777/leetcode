"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def maxSumSubmatrix(self, matrix: list[list[int]], k: int) -> int:
        row_count = len(matrix)
        col_count = len(matrix[0])
        max_sum = -(2**63 - 1)

        for i0 in range(col_count):
            sums = [0] * row_count
            for i1 in range(i0, col_count):
                for i2 in range(row_count):
                    sums[i2] += matrix[i2][i1]

                for i2 in range(row_count):
                    sum = 0
                    for i3 in range(i2, row_count):
                        sum += sums[i3]
                        if (sum > max_sum) and (sum <= k):
                            max_sum = sum
        return max_sum


class Input:
    matrix: list[list[int]]
    k: int

    def __init__(self, matrix: list[list[int]], k: int):
        self.matrix = matrix
        self.k = k


def main():
    inputs: list[Input] = [
        Input([[1, 0, 1], [0, -2, 3]], 2),
        Input([[2, 2, -1]], 3),
    ]

    solution = Solution()
    for input in inputs:
        matrix = input.matrix
        k = input.k
        result = solution.maxSumSubmatrix(matrix, k)
        print(result)


if __name__ == "__main__":
    main()
