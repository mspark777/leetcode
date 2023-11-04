from __future__ import annotations
from typing import List


class Solution:
    def getLastMoment(self, n: int, left: List[int], right: List[int]) -> int:
        left.append(-1)
        right.append(n + 1)
        lmax = max(left)
        rmax = max([n - r for r in right])

        return max(lmax, rmax)


def main():
    inputs = (
        (4, [4, 3], [0, 1]),
        (7, [], [0, 1, 2, 3, 4, 5, 6, 7]),
        (7, [0, 1, 2, 3, 4, 5, 6, 7], []),
    )

    for n, left, right in inputs:
        result = Solution().getLastMoment(n, left, right)
        print(result)


if __name__ == "__main__":
    main()
