"""
main
"""

from __future__ import annotations
from typing import Optional, Reversible


class Solution:
    def diagonalSort(self, mat: list[list[int]]) -> list[list[int]]:
        row_count = len(mat)
        col_count = len(mat[0])

        queues: dict[int, list[int]] = {}
        for i in range(row_count):
            for j in range(col_count):
                key = i - j
                if key not in queues:
                    queues[key] = []

                queues[key].append(mat[i][j])

        for queue in queues.values():
            queue.sort(reverse=True)

        result = [[]] * row_count
        for i in range(row_count):
            row = [0] * col_count
            for j in range(col_count):
                key = i - j
                queue = queues[key]
                row[j] = queue.pop()
            result[i] = row

        return result


class Input:
    mat: list[list[int]]

    def __init__(self, mat: list[list[int]]):
        self.mat = mat


def main():
    inputs: list[Input] = [
        Input([[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]]),
        Input(
            [
                [11, 25, 66, 1, 69, 7],
                [23, 55, 17, 45, 15, 52],
                [75, 31, 36, 44, 58, 8],
                [22, 27, 33, 25, 68, 4],
                [84, 28, 14, 11, 5, 50],
            ]
        ),
    ]

    solution = Solution()
    for input in inputs:
        mat = input.mat
        result = solution.diagonalSort(mat)
        print(result)


if __name__ == "__main__":
    main()
