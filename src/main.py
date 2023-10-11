from __future__ import annotations
from typing import Optional, List


class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        row = len(board)
        col = len(board[0])

        for r in range(row):
            if board[r][0] == "O":
                self.dfs(board, r, 0, row, col)
            if board[r][col - 1] == "O":
                self.dfs(board, r, col - 1, row, col)

        for c in range(col):
            if board[0][c] == "O":
                self.dfs(board, 0, c, row, col)
            if board[row - 1][c] == "O":
                self.dfs(board, row - 1, c, row, col)

        for r in range(row):
            for c in range(col):
                if board[r][c] == "O":
                    board[r][c] = "X"
                elif board[r][c] == "#":
                    board[r][c] = "O"

    def dfs(self, board: list[list[str]], r: int, c: int, row: int, col: int) -> None:
        if (r < 0) or (c < 0):
            return
        elif (r >= row) or (c >= col):
            return
        elif board[r][c] != "O":
            return

        board[r][c] = "#"
        self.dfs(board, r - 1, c, row, col)
        self.dfs(board, r + 1, c, row, col)
        self.dfs(board, r, c - 1, row, col)
        self.dfs(board, r, c + 1, row, col)


def main():
    inputs = (
        [
            ["X", "X", "X", "X"],
            ["X", "O", "O", "X"],
            ["X", "X", "O", "X"],
            ["X", "O", "X", "X"],
        ],
        [["X"]],
        [["X", "O", "X"], ["O", "X", "O"], ["X", "O", "X"]],
    )

    for board in inputs:
        Solution().solve(board)
        print(board)


if __name__ == "__main__":
    main()
