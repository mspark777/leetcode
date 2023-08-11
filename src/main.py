from __future__ import annotations
from typing import List


class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda i: i[0])

        result: list[list[int]] = [intervals[0]]
        for interval in intervals[1:]:
            [start, end] = interval
            last = result[-1]
            last_end = last[1]
            if last_end < start:
                result.append(interval)
            else:
                last[1] = max(last_end, end)

        return result


def main():
    inputs = [[[1, 3], [2, 6], [8, 10], [15, 18]], [[1, 4], [4, 5]]]

    for intervals in inputs:
        solution = Solution()
        result = solution.merge(intervals)
        print(result)


if __name__ == "__main__":
    main()
