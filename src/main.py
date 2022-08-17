"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def uniqueMorseRepresentations(self, words: list[str]) -> int:
        codes = [
            ".-",
            "-...",
            "-.-.",
            "-..",
            ".",
            "..-.",
            "--.",
            "....",
            "..",
            ".---",
            "-.-",
            ".-..",
            "--",
            "-.",
            "---",
            ".--.",
            "--.-",
            ".-.",
            "...",
            "-",
            "..-",
            "...-",
            ".--",
            "-..-",
            "-.--",
            "--..",
        ]
        seen = {"".join([codes[ord(ch) - ord("a")] for ch in word]) for word in words}
        return len(seen)


class Input:
    words: list[str]

    def __init__(self, words: list[str]):
        self.words = words


def main():
    inputs: list[Input] = [
        Input(["gin", "zen", "gig", "msg"]),
        Input(["a"]),
    ]

    solution = Solution()
    for i in inputs:
        result = solution.uniqueMorseRepresentations(i.words)
        print(result)


if __name__ == "__main__":
    main()
