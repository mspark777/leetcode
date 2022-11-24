from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        for r, row in enumerate(board):
            for c, w in enumerate(row):
                if w == word[0]:
                    seens = set()
                    if self.dfs(seens, board, r, c, 0, word):
                        return True

        return False

    def dfs(
        self,
        seens: set[str],
        board: list[list[str]],
        row: int,
        col: int,
        length: int,
        word: str,
    ) -> bool:
        if length == len(word):
            return True

        if (row < 0) or (col < 0):
            return False
        elif (row >= len(board)) or (col >= len(board[0])):
            return False

        seen = "{}|{}".format(row, col)
        if seen in seens:
            return False
        elif board[row][col] != word[length]:
            return False

        seens.add(seen)
        found = (
            self.dfs(seens, board, row + 1, col, length + 1, word)
            or self.dfs(seens, board, row - 1, col, length + 1, word)
            or self.dfs(seens, board, row, col + 1, length + 1, word)
            or self.dfs(seens, board, row, col - 1, length + 1, word)
        )

        seens.remove(seen)
        return found


def main():
    inputs: list[tuple[list[list[str]], str]] = [
        ([["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]], "ABCCED"),
        ([["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]], "SEE"),
        ([["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]], "ABCB"),
    ]

    solution = Solution()
    for board, word in inputs:
        result = solution.exist(board, word)
        print(result)


if __name__ == "__main__":
    main()
