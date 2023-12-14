from __future__ import annotations
from typing import List


class Solution:
    def findDiagonalOrder(self, mat: List[List[int]]) -> List[int]:
        row_count = len(mat)
        col_count = len(mat[0])
        result: list[int] = []
        is_up = True
        row = 0
        col = 0

        while row < row_count and col < col_count:
            result.append(mat[row][col])
            next_row = row + (-1 if is_up else 1)
            next_col = col + (1 if is_up else -1)

            if (
                next_row < 0
                or next_row >= row_count
                or next_col < 0
                or next_col >= col_count
            ):
                if is_up:
                    if next_col >= col_count:
                        row += 1
                    else:
                        col += 1
                else:
                    if next_row >= row_count:
                        col += 1
                    else:
                        row += 1

                is_up = not is_up
            else:
                row = next_row
                col = next_col

        return result


def main():
    inputs = ([[1, 2, 3], [4, 5, 6], [7, 8, 9]], [[1, 2], [3, 4]])

    for mat in inputs:
        result = Solution().findDiagonalOrder(mat)
        print(result)


if __name__ == "__main__":
    main()
