from __future__ import annotations
from typing import List


class Solution:
    def possibleBipartition(self, n: int, dislikes: List[List[int]]) -> bool:
        pass


class Input:
    n: int
    dislikes: list[list[int]]

    def __init__(self, n: int, dislikes: list[list[int]]):
        self.n = n
        self.dislikes = dislikes


def main():
    inputs: list[Input] = [
        Input(4, [[1, 2], [1, 3], [2, 4]]),
        Input(3, [[1, 2], [1, 3], [2, 3]]),
        Input(5, [[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.possibleBipartition(input.n, input.dislikes)
        print(result)


if __name__ == "__main__":
    main()
