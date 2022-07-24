"""
solution
"""
from __future__ import annotations

class Solution:
    def searchMatrix(self, matrix: list[list[int]], target: int) -> bool:
        row = len(matrix) - 1
        col = 0
        countcol = len(matrix[0])

        while (row >= 0) and (col < countcol):
            n = matrix[row][col]
            if target > n:
                col += 1
            elif target < n:
                row -= 1
            else:
                return True
        return False
