from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Solution:
    def makeGood(self, s: str) -> str:
        chars = list(s)

        j = 0
        for i in range(len(chars)):
            if j > 0:
                cur = chars[i]
                next = chars[j - 1]
                diff = abs(ord(cur) - ord(next))
                if diff == 32:
                    j -= 1
                    continue

            chars[j] = chars[i]
            j += 1

        return "".join(chars[0:j])


def main():
    inputs: list[str] = ["leEeetcode", "abBAcC", "s"]

    solution = Solution()
    for s in inputs:
        result = solution.makeGood(s)
        print(result)


if __name__ == "__main__":
    main()
