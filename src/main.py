"""
main
"""

from __future__ import annotations
from typing import Optional
from queue import PriorityQueue


class Solution:
    def maxPerformance(
        self, n: int, speeds: list[int], efficiencies: list[int], k: int
    ) -> int:
        candidates = sorted(zip(efficiencies, speeds), key=lambda x: -x[0])
        speed_sum = 0
        result = 0
        queue: PriorityQueue[int] = PriorityQueue()

        for efficiency, speed in candidates:
            queue.put(speed)
            speed_sum += speed
            if queue.qsize() > k:
                speed_sum -= queue.get()
            result = max(result, speed_sum * efficiency)

        return result % ((10**9) + 7)


class Input:
    n: int
    k: int
    speed: list[int]
    efficiency: list[int]

    def __init__(self, n: int, k: int, speed: list[int], efficiency: list[int]):
        self.n = n
        self.k = k
        self.speed = speed
        self.efficiency = efficiency


def main():
    inputs: list[Input] = [
        Input(n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=2),
        Input(n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=3),
        Input(n=6, speed=[2, 10, 3, 1, 5, 8], efficiency=[5, 4, 3, 9, 7, 2], k=4),
    ]

    solution = Solution()
    for input in inputs:
        n = input.n
        k = input.k
        speed = input.speed
        efficiency = input.efficiency
        result = solution.maxPerformance(n, speed, efficiency, k)
        print(result)


if __name__ == "__main__":
    main()
