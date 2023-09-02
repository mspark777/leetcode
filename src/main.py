from __future__ import annotations
from typing import List


class TrieNode:
    children: dict[str, TrieNode]
    is_word: bool

    def __init__(self):
        self.children = {}
        self.is_word = False


class Solution:
    def minExtraChar(self, s: str, dictionary: List[str]) -> int:
        slen = len(s)
        root = self.build(dictionary)
        dp = [0 for _ in range(slen + 1)]

        for start in range(slen - 1, -1, -1):
            dp[start] = dp[start + 1] + 1
            node = root
            for end in range(start, slen):
                if s[end] not in node.children:
                    break
                node = node.children[s[end]]
                if node.is_word:
                    dp[start] = min(dp[start], dp[end + 1])
        return dp[0]

    def build(self, dictionary: list[str]):
        root = TrieNode()
        for word in dictionary:
            node = root
            for ch in word:
                if ch not in node.children:
                    node.children[ch] = TrieNode()
                node = node.children[ch]
            node.is_word = True
        return root


def main():
    inputs = [
        ("leetscode", ["leet", "code", "leetcode"]),
        ("sayhelloworld", ["hello", "world"]),
    ]

    for s, dictionary in inputs:
        solution = Solution()
        result = solution.minExtraChar(s, dictionary)
        print(result)


if __name__ == "__main__":
    main()
