from __future__ import annotations
from typing import List


class Solution:
    def minTaps(self, n: int, ranges: List[int]) -> int:
        max_reach = [0 for _ in range(n + 1)]
        for i, r in enumerate(ranges):
            start = max(0, i - r)
            end = min(n, i + r)
            max_reach[start] = max(max_reach[start], end)

        taps = 0
        curr_end = 0
        next_end = 0
        for i in range(n + 1):
            if i > next_end:
                return -1

            if i > curr_end:
                taps += 1
                curr_end = next_end

            next_end = max(next_end, max_reach[i])

        return taps


def main():
    inputs = [(5, [3, 4, 1, 1, 0, 0]), (3, [0, 0, 0, 0])]

    for n, ranges in inputs:
        solution = Solution()
        result = solution.minTaps(n, ranges)
        print(result)


if __name__ == "__main__":
    main()
