from __future__ import annotations
from typing import List


class Solution:
    def wordBreak(self, s: str, word_dict: List[str]) -> bool:
        words = set(word_dict)
        checks = [False for _ in range(len(s) + 1)]
        checks[0] = True

        for right in range(1, len(s) + 1):
            for left in range(0, right):
                if not checks[left]:
                    continue

                sub = s[left:right]
                if sub in words:
                    checks[right] = True
                    break

        return checks[len(s)]


def main():
    inputs = [
        ("leetcode", ["leet", "code"]),
        ("applepenapple", ["apple", "pen"]),
        ("catsandog", ["cats", "dog", "sand", "and", "cat"]),
    ]

    for s, word_dict in inputs:
        solution = Solution()
        result = solution.wordBreak(s, word_dict)
        print(result)


if __name__ == "__main__":
    main()
