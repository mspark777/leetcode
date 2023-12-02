from __future__ import annotations
from typing import List
from collections import defaultdict


class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        counts = defaultdict[str, int](int)

        for ch in chars:
            counts[ch] += 1

        result = 0

        for word in words:
            wcounts = defaultdict[str, int](int)
            for w in word:
                wcounts[w] += 1

            good = True
            for c, freq in wcounts.items():
                if counts[c] < freq:
                    good = False
                    break

            if good:
                result += len(word)

        return result


def main():
    inputs = (
        (["cat", "bt", "hat", "tree"], "atach"),
        (["hello", "world", "leetcode"], "welldonehoneyr"),
    )

    for words, chars in inputs:
        result = Solution().countCharacters(words, chars)
        print(result)


if __name__ == "__main__":
    main()
