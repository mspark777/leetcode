from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Solution:
    def reverseVowels(self, s: str) -> str:
        vowels = set(["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"])
        words = list(s)
        left = 0
        right = len(words) - 1

        while left < right:
            l = words[left]
            if l not in vowels:
                left += 1
                continue

            r = words[right]
            if r not in vowels:
                right -= 1
                continue

            words[left] = r
            words[right] = l
            left += 1
            right -= 1

        return "".join(words)


def main():
    inputs: list[str] = ["hello", "leetcode"]

    solution = Solution()
    for s in inputs:
        result = solution.reverseVowels(s)
        print(result)


if __name__ == "__main__":
    main()
