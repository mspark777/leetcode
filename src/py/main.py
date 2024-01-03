from __future__ import annotations
from typing import List


class Solution:
    def findPoisonedDuration(self, time_series: List[int], duration: int) -> int:
        n = len(time_series)
        if n < 1:
            return 0

        result = 0
        for i in range(n - 1):
            diff = time_series[i + 1] - time_series[i]
            result += min(diff, duration)

        result += duration

        return result


def main():
    input = (([1, 4], 2), ([1, 2], 2))

    for time_series, duration in input:
        result = Solution().findPoisonedDuration(time_series, duration)
        print(result)


if __name__ == "__main__":
    main()
