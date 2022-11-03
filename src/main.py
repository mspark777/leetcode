from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Solution:
    def longestPalindrome(self, words: List[str]) -> int:
        counts = Counter(words)
        result = 0
        central = False

        for word, count in counts.items():
            first = word[0]
            second = word[1]
            if first == second:
                if (count % 2) == 0:
                    result += count
                else:
                    result += count - 1
                    central = True
            elif first < second:
                rword = f"{second}{first}"
                if rword in counts:
                    result += 2 * min(count, counts[rword])

        if central:
            result += 1

        return result * 2


def main():
    inputs: list[list[str]] = [
        ["lc", "cl", "gg"],
        ["ab", "ty", "yt", "lc", "cl", "ab"],
        ["cc", "ll", "xx"],
    ]

    solution = Solution()
    for words in inputs:
        result = solution.longestPalindrome(words)
        print(result)


if __name__ == "__main__":
    main()
