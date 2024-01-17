from __future__ import annotations
from typing import List
from collections import Counter


class Solution:
    def uniqueOccurrences(self, arr: List[int]) -> bool:
        counts = Counter(arr)
        return len(set(counts.values())) == len(counts)


def main():
    input = ([1, 2, 2, 1, 1, 3], [1, 2], [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0])

    for arr in input:
        result = Solution().uniqueOccurrences(arr)
        print(result)


if __name__ == "__main__":
    main()
