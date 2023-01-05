from __future__ import annotations
from typing import Counter, List


class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        points.sort(key=lambda a: a[1])

        result = 1
        prev = 0

        for cur in range(1, len(points)):
            if points[cur][0] > points[prev][1]:
                result += 1
                prev = cur

        return result


def main():
    inputs: list[list[list[int]]] = [
        [[10, 16], [2, 8], [1, 6], [7, 12]],
        [[1, 2], [3, 4], [5, 6], [7, 8]],
        [[1, 2], [2, 3], [3, 4], [4, 5]],
    ]

    solution = Solution()
    for points in inputs:
        result = solution.findMinArrowShots(points)
        print(result)


if __name__ == "__main__":
    main()
