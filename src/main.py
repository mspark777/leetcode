from __future__ import annotations
from typing import Callable, Optional, List
from collections import Counter, deque


class Solution:
    def computeArea(
        self,
        ax1: int,
        ay1: int,
        ax2: int,
        ay2: int,
        bx1: int,
        by1: int,
        bx2: int,
        by2: int,
    ) -> int:
        overx = min(ax2, bx2) - max(ax1, bx1)
        overy = min(ay2, by2) - max(ay1, by1)

        areaa = (ay2 - ay1) * (ax2 - ax1)
        areab = (by2 - by1) * (bx2 - bx1)
        areac = overx * overy if (overx > 0) and (overy > 0) else 0

        return abs(areaa) + abs(areab) - areac


def main():
    global guess

    inputs: list[list[int]] = [[-3, 0, 3, 4, 0, -1, 9, 2], [-2, -2, 2, 2, -2, -2, 2, 2]]

    solution = Solution()
    for [ax1, ay1, ax2, ay2, bx1, by1, bx2, by2] in inputs:
        result = solution.computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2)
        print(result)


if __name__ == "__main__":
    main()
