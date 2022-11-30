from __future__ import annotations
from collections import Counter
from typing import List


class Solution:
    def uniqueOccurrences(self, arr: List[int]) -> bool:
        counts = Counter(arr)
        occurrences = set(counts.values())

        return len(counts) == len(occurrences)


def main():
    inputs: list[list[int]] = [
        [1, 2, 2, 1, 1, 3],
        [1, 2],
        [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0],
    ]

    solution = Solution()
    for arr in inputs:
        result = solution.uniqueOccurrences(arr)
        print(result)


if __name__ == "__main__":
    main()
