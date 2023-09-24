from __future__ import annotations
from typing import Optional, List


class Solution:
    def champagneTower(self, poured: int, query_row: int, query_glass: int) -> float:
        rows = [[0.0] * k for k in range(1, 102)]
        rows[0][0] = poured
        for r in range(query_row + 1):
            for c in range(r + 1):
                q = (rows[r][c] - 1.0) / 2.0
                if q > 0:
                    rows[r + 1][c] += q
                    rows[r + 1][c + 1] += q

        return min(1, rows[query_row][query_glass])


def main():
    inputs = [(1, 1, 1), (2, 1, 1), (100000009, 33, 17)]

    for poured, query_row, query_glass in inputs:
        solution = Solution()
        result = solution.champagneTower(poured, query_row, query_glass)
        print(result)


if __name__ == "__main__":
    main()
