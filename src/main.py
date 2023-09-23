from __future__ import annotations
from typing import Optional, List


class Solution:
    def longestStrChain(self, words: List[str]) -> int:
        words.sort(key=lambda x: len(x))
        result = 0

        dp = dict[str, int]()
        for word in words:
            cur_len = 1
            for i in range(len(word)):
                pre = word[0:i] + word[i + 1 :]
                if pre in dp:
                    cur_len = max(cur_len, dp[pre] + 1)

            dp[word] = cur_len
            result = max(result, cur_len)

        return result


def main():
    inputs = [
        ["a", "b", "ba", "bca", "bda", "bdca"],
        ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"],
        ["abcd", "dbqca"],
        ["bdca", "bda", "ca", "dca", "a"],
    ]

    for words in inputs:
        solution = Solution()
        result = solution.longestStrChain(words)
        print(result)


if __name__ == "__main__":
    main()
