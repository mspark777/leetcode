"""
main
"""

from __future__ import annotations
from queue import PriorityQueue
from typing import Optional


class Solution:
    def minRefuelStops(
        self, target: int, start_fuel: int, stations: list[list[int]]
    ) -> int:
        queue: PriorityQueue[tuple[int, int]] = PriorityQueue()
        result = 0
        prev = 0
        tank = start_fuel
        for station in stations:
            position = station[0]
            tank -= position - prev
            while (not queue.empty()) and (tank < 0):
                (_, fuel) = queue.get()
                tank += fuel
                result += 1

            if tank < 0:
                return -1

            fuel = station[1]
            queue.put((-fuel, fuel))
            prev = position

        tank -= target - prev
        while (not queue.empty()) and (tank < 0):
            (_, fuel) = queue.get()
            tank += fuel
            result += 1
        if tank < 0:
            return -1

        return result


class Input:
    target: int
    start_fuel: int
    stations: list[list[int]]

    def __init__(self, target: int, start_fuel: int, stations: list[list[int]]):
        self.target = target
        self.start_fuel = start_fuel
        self.stations = stations


def main():
    inputs: list[Input] = [
        Input(1, 1, []),
        Input(100, 1, [[10, 100]]),
        Input(100, 10, [[10, 60], [20, 30], [30, 30], [60, 40]]),
        Input(100, 50, [[50, 50]]),
        Input(100, 50, [[25, 50], [50, 25]]),
    ]

    solution = Solution()
    for i in inputs:
        target = i.target
        start_fuel = i.start_fuel
        stations = i.stations
        result = solution.minRefuelStops(target, start_fuel, stations)
        print(result)


if __name__ == "__main__":
    main()
