from __future__ import annotations
from typing import List


class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda i: i[1])

        result = 0
        k = -(2**31)

        for [x, y] in intervals:
            if x >= k:
                k = y
            else:
                result += 1

        return result


def main():
    inputs = [
        [[1, 2], [2, 3], [3, 4], [1, 3]],
        [[1, 2], [1, 2], [1, 2]],
        [[1, 2], [2, 3]],
    ]

    for intervals in inputs:
        solution = Solution()
        result = solution.eraseOverlapIntervals(intervals)
        print(result)


if __name__ == "__main__":
    main()
