"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        return self.transform(s) == self.transform(t)

    def transform(self, s: str) -> str:
        index_mapping: dict[str, int] = {}
        result: list[str] = []

        for i, ch in enumerate(s):
            if ch not in index_mapping:
                index_mapping[ch] = i
            result.append(str(index_mapping[ch]))

        return " ".join(result)


class Input:
    s: str
    t: str

    def __init__(self, s: str, t: str):
        self.s = s
        self.t = t


def main():
    inputs: list[Input] = [
        Input("egg", "add"),
        Input("foo", "bar"),
        Input("paper", "title"),
    ]

    solution = Solution()
    for i in inputs:
        s = i.s
        t = i.t
        result = solution.isIsomorphic(s, t)
        print(result)


if __name__ == "__main__":
    main()
