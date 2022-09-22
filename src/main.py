"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def reverseWords(self, s: str) -> str:
        return " ".join([word[::-1] for word in s.split()])


class Input:
    s: str

    def __init__(self, s: str):
        self.s = s


def main():
    inputs: list[Input] = [
        Input("Let's take LeetCode contest"),
        Input("God Ding"),
    ]

    solution = Solution()
    for input in inputs:
        s = input.s
        result = solution.reverseWords(s)
        print(result)


if __name__ == "__main__":
    main()
