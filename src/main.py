from __future__ import annotations
from typing import Optional, List


class Solution:
    def wordBreak(self, s: str, words: List[str]) -> List[str]:
        word_set = set(words)
        n = len(s)
        dp: list[list[str]] = [[] for _ in range(n + 1)]
        dp[0].append("")

        for i in range(n):
            for j in range(i + 1, n + 1):
                temp = s[i:j]
                if temp in word_set:
                    for t in dp[i]:
                        dp[j].append(t + (" " if t != "" else t) + temp)

        return dp[n]


def main():
    inputs = (
        ("catsanddog", ["cat", "cats", "and", "sand", "dog"]),
        ("pineapplepenapple", ["apple", "pen", "applepen", "pine", "pineapple"]),
        ("catsandog", ["cats", "dog", "sand", "and", "cat"]),
    )

    for s, words in inputs:
        result = Solution().wordBreak(s, words)
        print(result)


if __name__ == "__main__":
    main()
