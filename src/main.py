from __future__ import annotations
from typing import Callable, Optional, List
from collections import Counter, deque


guess: Callable[[int], int] = lambda n: n


def getGuess(pick: int) -> Callable[[int], int]:
    def g(n: int):
        if n < pick:
            return 1
        elif n > pick:
            return -1
        else:
            return 0

    return g


class Solution:
    def guessNumber(self, n: int) -> int:
        left = 1
        right = n
        while left <= right:
            m = (left + right) // 2
            res = guess(m)
            if res < 0:
                right = m - 1
            elif res > 0:
                left = m + 1
            else:
                return m

        return -1


def main():
    global guess

    inputs: list[tuple[int, int]] = [(10, 6), (1, 1), (2, 1)]

    solution = Solution()
    for n, pick in inputs:
        guess = getGuess(pick)
        result = solution.guessNumber(n)
        print(result)


if __name__ == "__main__":
    main()
