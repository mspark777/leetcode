from __future__ import annotations
from typing import List

QUEEN = "Q"
EMPTY = "."


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result: list[list[str]] = []
        board = [[EMPTY for c in range(n)] for r in range(n)]
        self.solve(0, board, result)

        return result

    def solve(self, col: int, board: list[list[str]], result: list[list[str]]):
        if col >= len(board):
            result.append(["".join(r) for r in board])
            return

        for r in range(len(board)):
            if self.is_safe(r, col, board):
                board[r][col] = QUEEN
                self.solve(col + 1, board, result)
                board[r][col] = EMPTY

    def is_safe(self, row: int, col: int, board: list[list[str]]) -> bool:
        r = row
        c = col
        while (r >= 0) and (c >= 0):
            if board[r][c] == QUEEN:
                return False
            r -= 1
            c -= 1

        for c in range(col, -1, -1):
            if board[row][c] == QUEEN:
                return False

        r = row
        c = col
        while (r < len(board)) and (c >= 0):
            if board[r][c] == QUEEN:
                return False
            r += 1
            c -= 1

        return True


def main():
    inputs = [4, 1]

    for n in inputs:
        solution = Solution()
        result = solution.solveNQueens(n)
        print(result)


if __name__ == "__main__":
    main()
