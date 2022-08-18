"""
main
"""

from __future__ import annotations
from typing import Optional
from collections import Counter


class Solution:
    def minSetSize(self, arr: list[int]) -> int:
        freqs = Counter(arr)
        pqueue = [x for x in freqs.values()]
        pqueue.sort(reverse=True)

        deleted = 0
        result = 0
        half = len(arr) // 2
        for freq in pqueue:
            deleted += freq
            result += 1
            if deleted >= half:
                return result
        return -1


class Input:
    arr: list[int]

    def __init__(self, arr: list[int]):
        self.arr = arr


def main():
    inputs: list[Input] = [
        Input([3, 3, 3, 3, 5, 5, 5, 2, 2, 7]),
        Input([7, 7, 7, 7, 7, 7]),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.minSetSize(i.arr)
        print(result)


if __name__ == "__main__":
    main()
