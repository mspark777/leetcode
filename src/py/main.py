from __future__ import annotations
from typing import List


class Solution:
    def eliminateMaximum(self, dist: List[int], speed: List[int]) -> int:
        arrival: list[float] = []
        for d, s in zip(dist, speed):
            arrival.append(d / s)

        arrival.sort()
        result = 0

        for i, val in enumerate(arrival):
            if val <= i:
                break
            else:
                result += 1

        return result


def main():
    inputs = (
        ([1, 3, 4], [1, 1, 1]),
        ([1, 1, 2, 3], [1, 1, 1, 1]),
        ([3, 2, 4], [5, 3, 2]),
    )

    for dist, speed in inputs:
        result = Solution().eliminateMaximum(dist, speed)
        print(result)


if __name__ == "__main__":
    main()
