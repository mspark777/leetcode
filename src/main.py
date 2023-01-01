from __future__ import annotations
from typing import List


class Solution:
    def reverseString(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        i = 0
        j = len(s) - 1
        while i < j:
            s[i], s[j] = s[j], s[i]
            i += 1
            j -= 1


def main():
    inputs: list[list[str]] = [
        ["h", "e", "l", "l", "o"],
        ["H", "a", "n", "n", "a", "h"],
    ]

    solution = Solution()
    for s in inputs:
        solution.reverseString(s)
        print(s)


if __name__ == "__main__":
    main()
