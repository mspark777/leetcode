from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def nearestExit(self, maze: List[List[str]], entrance: List[int]) -> int:
        WALL = "+"
        row_count = len(maze)
        col_count = len(maze[0])
        last_row = row_count - 1
        last_col = col_count - 1
        dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]]

        queue: Deque[list[int]] = deque([[entrance[0], entrance[1], 0]])
        maze[entrance[0]][entrance[1]] = WALL
        while queue:
            [row, col, steps] = queue.popleft()
            next_steps = steps + 1

            for i in range(len(dirs)):
                [r, c] = dirs[i]
                next_row = row + r
                next_col = col + c
                if next_row < 0:
                    continue
                elif next_row >= row_count:
                    continue
                elif next_col < 0:
                    continue
                elif next_col >= col_count:
                    continue
                elif maze[next_row][next_col] == WALL:
                    continue

                if next_row == 0:
                    return next_steps
                elif next_row == last_row:
                    return next_steps
                elif next_col == 0:
                    return next_steps
                elif next_col == last_col:
                    return next_steps

                maze[next_row][next_col] = WALL
                queue.append([next_row, next_col, next_steps])

        return -1


class Input:
    maze: list[list[str]]
    entrance: list[int]

    def __init__(self, maze: list[list[str]], entrance: list[int]):
        self.maze = maze
        self.entrance = entrance


def main():

    inputs: list[Input] = [
        Input(
            [["+", "+", ".", "+"], [".", ".", ".", "+"], ["+", "+", "+", "."]], [1, 2]
        ),
        Input([["+", "+", "+"], [".", ".", "."], ["+", "+", "+"]], [1, 0]),
        Input([[".", "+"]], [0, 0]),
    ]

    solution = Solution()
    for input in inputs:
        maze = input.maze
        entrance = input.entrance
        result = solution.nearestExit(maze, entrance)
        print(result)


if __name__ == "__main__":
    main()
