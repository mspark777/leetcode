from __future__ import annotations
from typing import Optional, List


class Solution:
    def arrayStringsAreEqual(self, word1: List[str], word2: List[str]) -> bool:
        return "".join(word1) == "".join(word2)


def main():
    inputs: list[tuple[list[str], list[str]]] = [
        (["ab", "c"], ["a", "bc"]),
        (["a", "cb"], ["ab", "c"]),
        (["abc", "d", "defg"], ["abcddefg"]),
        (["abc", "d", "defg"], ["abcddef"]),
    ]

    solution = Solution()
    for word1, word2 in inputs:
        result = solution.arrayStringsAreEqual(word1, word2)
        print(result)


if __name__ == "__main__":
    main()
