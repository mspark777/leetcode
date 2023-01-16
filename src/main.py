from __future__ import annotations
from typing import List
import bisect


class Solution:
    def insert(
        self, intervals: List[List[int]], new_interval: List[int]
    ) -> List[List[int]]:
        position = bisect.bisect(intervals, new_interval)
        intervals.insert(position, new_interval)

        result = []
        for i in range(len(intervals)):
            if not result or (intervals[i][0] > result[-1][1]):
                result.append(intervals[i])
            else:
                result[-1][1] = max(result[-1][1], intervals[i][1])

        return result


def main():
    inputs: list[tuple[list[list[int]], list[int]]] = [
        ([[1, 3], [6, 9]], [2, 5]),
        ([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], [4, 8]),
    ]

    for intervals, newInterval in inputs:
        solution = Solution()
        result = solution.insert(intervals, newInterval)
        print(result)


if __name__ == "__main__":
    main()
