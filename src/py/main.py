from __future__ import annotations
from typing import List


class Solution:
    def numMagicSquaresInside(self, grid: List[List[int]]) -> int:
        result = 0
        m = len(grid)
        n = len(grid[0])
        for row in range(m - 2):
            for col in range(n - 2):
                if self.is_magic_square(grid, row, col):
                    result += 1

        return result

    def is_magic_square(self, grid: list[list[int]], row: int, col: int) -> bool:
        sequence = "2943816729438167"
        sequence_reversed = "7618349276183492"

        border: list[str] = []
        border_indices = [0, 1, 2, 5, 8, 7, 6, 3]
        for i in border_indices:
            num = grid[row + i // 3][col + (i % 3)]
            border.append(str(num))

        border_converted = "".join(border)
        return (
            grid[row][col] % 2 == 0
            and (
                sequence.find(border_converted) != -1
                or sequence_reversed.find(border_converted) != -1
            )
            and grid[row + 1][col + 1] == 5
        )


def main():
    inputs: list[list[list[int]]] = [[[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]], [[8]]]

    for input in inputs:
        result = Solution().numMagicSquaresInside(input)
        print(result)


if __name__ == "__main__":
    main()
