from __future__ import annotations
from typing import List
from heapq import heappop, heappush


class Solution:
    def smallestChair(self, times: List[List[int]], target_friend: int) -> int:
        sorted_times = sorted(
            [(arrival, leave, index) for index, (arrival, leave) in enumerate(times)]
        )

        next_chair = 0
        available_chairs: list[int] = []
        leaving_queue: list[tuple[int, int]] = []

        for arrival, leave, index in sorted_times:
            while leaving_queue and leaving_queue[0][0] <= arrival:
                _, chair = heappop(leaving_queue)
                heappush(available_chairs, chair)

            if available_chairs:
                current_chair = heappop(available_chairs)
            else:
                current_chair = next_chair
                next_chair += 1

            heappush(leaving_queue, (leave, current_chair))

            if index == target_friend:
                return current_chair

        return 0


def main():
    inputs = [([[1, 4], [2, 3], [4, 6]], 1), ([[3, 10], [1, 5], [2, 6]], 0)]

    for times, target in inputs:
        result = Solution().smallestChair(times, target)
        print(result)


if __name__ == "__main__":
    main()
