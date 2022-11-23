from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        seens: set[str] = set()

        for r in range(9):
            for c in range(9):
                n = board[r][c]
                if n == ".":
                    continue

                ns = "({})".format(n)
                row = "{}{}".format(ns, r)
                col = "{}{}".format(c, ns)
                cross = "{}{}{}".format(r // 3, ns, c // 3)
                if (row in seens) or (col in seens) or (cross in seens):
                    return False

                seens.add(row)
                seens.add(col)
                seens.add(cross)

        return True


def main():
    inputs: list[list[list[str]]] = [
        [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ],
        [
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ],
    ]

    solution = Solution()
    for board in inputs:
        result = solution.isValidSudoku(board)
        print(result)


if __name__ == "__main__":
    main()
