"""
main
"""

from __future__ import annotations
from collections import Counter
from typing import Optional


class Solution:
    def firstUniqChar(self, s: str) -> int:
        counter = Counter(s)

        for i, ch in enumerate(s):
            if counter[ch] == 1:
                return i
        return -1


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs: list[Input] = [
        Input("leetcode"),
        Input("loveleetcode"),
        Input("aabb"),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.firstUniqChar(i.s)
        print(result)


if __name__ == "__main__":
    main()
