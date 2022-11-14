from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Helper:
    unimap: dict[int, int]
    islands: int

    def __init__(self) -> None:
        self.unimap = {}
        self.islands = 0

    def find(self, x: int) -> int:
        if x not in self.unimap:
            self.unimap[x] = x
            self.islands += 1

        p = self.unimap[x]
        if x != p:
            self.unimap[x] = self.find(p)

        return self.unimap[x]

    def uni(self, x: int, y: int) -> None:
        x = self.find(x)
        y = self.find(y)
        if x != y:
            self.unimap[x] = y
            self.islands -= 1


class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        helper = Helper()
        for stone in stones:
            helper.uni(stone[0], ~stone[1])

        return len(stones) - helper.islands


def main():
    inputs: list[list[list[int]]] = [
        [[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]],
        [[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]],
        [
            [0, 0],
            [0, 1],
            [1, 0],
            [1, 1],
            [2, 1],
            [2, 2],
            [3, 2],
            [3, 3],
            [3, 4],
            [4, 3],
            [4, 4],
        ],
    ]

    solution = Solution()
    for stones in inputs:
        result = solution.removeStones(stones)
        print(result)


if __name__ == "__main__":
    main()
