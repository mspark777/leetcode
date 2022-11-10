from __future__ import annotations
from typing import Optional, List
from collections import Counter, deque


class Solution:
    def removeDuplicates(self, s: str) -> str:
        result: list[str] = []

        for c in s:
            if result and (result[-1] == c):
                result.pop()
            else:
                result.append(c)

        return "".join(result)


def main():
    inputs: list[str] = ["abbaca", "azxxzy"]

    solution = Solution()
    for s in inputs:
        result = solution.removeDuplicates(s)
        print(result)


if __name__ == "__main__":
    main()
