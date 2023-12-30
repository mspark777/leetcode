from __future__ import annotations
from typing import List
from collections import defaultdict


class Solution:
    def makeEqual(self, words: List[str]) -> bool:
        counts = defaultdict[str, int](int)

        for word in words:
            for ch in word:
                counts[ch] += 1

        word_count = len(words)
        for count in counts.values():
            if count % word_count != 0:
                return False

        return True


def main():
    input = (["abc", "aabc", "bc"], ["ab", "a"])

    for words in input:
        result = Solution().makeEqual(words)
        print(result)


if __name__ == "__main__":
    main()
