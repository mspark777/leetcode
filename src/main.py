from __future__ import annotations
from collections import Counter
from typing import List


class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        if len(word1) != len(word2):
            return False

        freq1 = Counter(word1)
        freq2 = Counter(word2)

        if len(freq1) != len(freq2):
            return False
        elif [k for k in freq1.keys() if k not in freq2]:
            return False

        counts1 = list(freq1.values())
        counts2 = list(freq2.values())

        counts1.sort(reverse=True)
        counts2.sort(reverse=True)

        for i, cnt1 in enumerate(counts1):
            if cnt1 != counts2[i]:
                return False

        return True


def main():
    inputs: list[list[str]] = [["abc", "bca"], ["a", "aa"], ["cabbba", "abbccc"]]

    solution = Solution()
    for [word1, word2] in inputs:
        result = solution.closeStrings(word1, word2)
        print(result)


if __name__ == "__main__":
    main()
