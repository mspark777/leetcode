from __future__ import annotations
from typing import List
from queue import PriorityQueue


class Solution:
    def minStoneSum(self, piles: List[int], k: int) -> int:
        queue = PriorityQueue[tuple[int, int]]()
        result = 0
        for pile in piles:
            result += pile
            queue.put((-pile, pile))

        for i in range(k):
            value = queue.get()[1]
            remove = value // 2
            value -= remove
            result -= remove
            queue.put((-value, value))

        return result


class Input:
    piles: list[int]
    k: int

    def __init__(self, piles: list[int], k: int) -> None:
        self.piles = piles
        self.k = k


def main():
    inputs: list[Input] = [
        Input([5, 4, 9], 2),
        Input([4, 3, 6, 7], 3),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.minStoneSum(input.piles, input.k)
        print(result)


if __name__ == "__main__":
    main()
