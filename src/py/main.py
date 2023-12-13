from __future__ import annotations
from typing import List


class Solution:
    def numSpecial(self, mat: List[List[int]]) -> int:
        m = len(mat)
        n = len(mat[0])
        rows = [0] * m
        cols = [0] * n
        for r in range(m):
            for c in range(n):
                rows[r] += mat[r][c]

        for c in range(n):
            for r in range(m):
                cols[c] += mat[r][c]

        result = 0
        for r, row in enumerate(mat):
            for c, col in enumerate(row):
                if col == 1 and rows[r] == 1 and cols[c] == 1:
                    result += 1

        return result


def main():
    inputs = ([[1, 0, 0], [0, 0, 1], [1, 0, 0]], [[1, 0, 0], [0, 1, 0], [0, 0, 1]])

    for nums in inputs:
        result = Solution().numSpecial(nums)
        print(result)


if __name__ == "__main__":
    main()
