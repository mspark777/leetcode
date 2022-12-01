from __future__ import annotations
from collections import Counter
from typing import List


class Solution:
    def halvesAreAlike(self, s: str) -> bool:
        vowels = set(["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"])
        first = 0
        second = 0

        i = 0
        for j in range(len(s) // 2, len(s)):
            if s[i] in vowels:
                first += 1

            if s[j] in vowels:
                second += 1

            i += 1

        return first == second


def main():
    inputs: list[str] = ["book", "textbook"]

    solution = Solution()
    for s in inputs:
        result = solution.halvesAreAlike(s)
        print(result)


if __name__ == "__main__":
    main()
