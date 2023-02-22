from __future__ import annotations
from typing import List


class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        total_load = 0
        max_load = 0

        for weight in weights:
            total_load += weight
            max_load = max(max_load, weight)

        left = max_load
        right = total_load

        while left < right:
            middle = (left + right) // 2
            if self.feasible(weights, middle, days):
                right = middle
            else:
                left = middle + 1

        return left

    def feasible(self, weights: list[int], capacity: int, days: int) -> bool:
        days_needed = 1
        current_load = 0

        for weight in weights:
            current_load += weight
            if current_load > capacity:
                days_needed += 1
                current_load = weight

            if days_needed > days:
                return False

        return True


def main():
    inputs: list[tuple[list[int], int]] = [
        ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
        ([3, 2, 2, 4, 1, 4], 3),
        ([1, 2, 3, 1, 1], 4),
    ]

    for weights, days in inputs:
        solution = Solution()
        result = solution.shipWithinDays(weights, days)
        print(result)


if __name__ == "__main__":
    main()
