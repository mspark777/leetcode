"""
main
"""

from __future__ import annotations
from typing import Optional
from collections import Counter


class Solution:
    def canConstruct(self, ransom_note: str, magazine: str) -> bool:
        return (Counter(ransom_note) - Counter(magazine)).total() == 0


class Input:
    ransom_note: str
    magazine: str

    def __init__(self, ransom_note: str, magazine: str):
        self.ransom_note = ransom_note
        self.magazine = magazine


def main():
    inputs: list[Input] = [Input("a", "b"), Input("aa", "ab"), Input("aa", "aab")]

    solution = Solution()
    for input in inputs:
        ransom_note = input.ransom_note
        magazine = input.magazine
        result = solution.canConstruct(ransom_note, magazine)
        print(result)


if __name__ == "__main__":
    main()
