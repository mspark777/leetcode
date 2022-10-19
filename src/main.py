from __future__ import annotations
from typing import Optional, List
from collections import Counter


class Solution:
    def topKFrequent(self, words: List[str], k: int) -> List[str]:
        counts = Counter(words)
        result = sorted(counts, key=lambda x: (-counts[x], x))
        return result[:k]


def main():
    inputs: list[tuple[list[str], int]] = [
        (["i", "love", "leetcode", "i", "love", "coding"], 2),
        (["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], 4),
    ]

    solution = Solution()
    for words, k in inputs:
        result = solution.topKFrequent(words, k)
        print(result)


if __name__ == "__main__":
    main()
