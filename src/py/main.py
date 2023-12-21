from __future__ import annotations
from typing import List


class Solution:
    def maxWidthOfVerticalArea(self, points: List[List[int]]) -> int:
        points.sort(key=lambda a: a[0])

        result = 0
        for i in range(1, len(points)):
            result = max(result, points[i][0] - points[i - 1][0])

        return result


def main():
    input = (
        [[8, 7], [9, 9], [7, 4], [9, 7]],
        [[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]],
    )

    for points in input:
        result = Solution().maxWidthOfVerticalArea(points)
        print(result)


if __name__ == "__main__":
    main()
