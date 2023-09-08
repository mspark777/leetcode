from __future__ import annotations
from typing import List


class Solution:
    def generate(self, num_rows: int) -> List[List[int]]:
        result: list[list[int]] = []

        for num_cols in range(num_rows):
            row = [0 for _ in range(num_cols + 1)]
            row[0] = 1
            row[-1] = 1
            prev = num_cols - 1
            for c in range(1, num_cols):
                row[c] = result[prev][c - 1] + result[prev][c]

            result.append(row)

        return result


def main():
    inputs = [4, 1]

    for numRows in inputs:
        solution = Solution()
        result = solution.generate(numRows)
        print(result)


if __name__ == "__main__":
    main()
