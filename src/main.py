from __future__ import annotations
from typing import List
import heapq


class Solution:
    def findMaximizedCapital(
        self, k: int, w: int, profits: List[int], capital: List[int]
    ) -> int:
        n = len(profits)
        projects = list(zip(capital, profits))
        projects.sort()

        queue: list[int] = []
        proj = 0

        for i in range(k):
            while (proj < n) and (projects[proj][0] <= w):
                heapq.heappush(queue, -projects[proj][1])
                proj += 1

            if not queue:
                break

            w -= heapq.heappop(queue)

        return w


class Input:
    k: int
    w: int
    profits: list[int]
    capital: list[int]

    def __init__(self, k: int, w: int, profits: list[int], capital: list[int]):
        self.k = k
        self.w = w
        self.profits = profits
        self.capital = capital


def main():
    inputs: list[Input] = [
        Input(k=2, w=0, profits=[1, 2, 3], capital=[0, 1, 1]),
        Input(k=3, w=0, profits=[1, 2, 3], capital=[0, 1, 2]),
    ]

    for input in inputs:
        solution = Solution()
        result = solution.findMaximizedCapital(
            input.k, input.w, input.profits, input.capital
        )
        print(result)


if __name__ == "__main__":
    main()
