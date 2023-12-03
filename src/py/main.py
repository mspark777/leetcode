from __future__ import annotations
from typing import List


class Solution:
    def minTimeToVisitAllPoints(self, points: List[List[int]]) -> int:
        result = 0

        for i in range(len(points) - 1):
            cx, cy = points[i]
            tx, ty = points[i + 1]
            result += max(abs(tx - cx), abs(ty - cy))

        return result


def main():
    inputs = ([[1, 1], [3, 4], [-1, 0]], [[3, 2], [-2, 2]])

    for points in inputs:
        result = Solution().minTimeToVisitAllPoints(points)
        print(result)


if __name__ == "__main__":
    main()
