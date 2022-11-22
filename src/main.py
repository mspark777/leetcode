from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque


class Solution:
    def numSquares(self, n: int) -> int:
        memos = [2**31 for i in range(n + 1)]

        memos[0] = 0
        cur = 1
        squire = 1

        while squire <= n:
            for i in range(squire, n + 1):
                memos[i] = min(memos[i - squire] + 1, memos[i])

            cur += 1
            squire = cur * cur

        return memos[n]


def main():

    inputs: list[int] = [12, 13]

    solution = Solution()
    for n in inputs:
        result = solution.numSquares(n)
        print(result)


if __name__ == "__main__":
    main()
