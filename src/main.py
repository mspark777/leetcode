from __future__ import annotations
from collections import Counter
from typing import Optional, List


class Solution:
    def removeDuplicateLetters(self, s: str) -> str:
        counts = Counter(s)
        result = list[str]()
        visited = set[str]()

        for ch in s:
            counts[ch] -= 1
            if ch in visited:
                continue
            while result and (ch < result[-1]) and (counts[result[-1]] > 0):
                visited.remove(result.pop())

            visited.add(ch)
            result.append(ch)

        return "".join(result)


def main():
    inputs = ["bcabc", "cbacdcbc"]

    for s in inputs:
        solution = Solution()
        result = solution.removeDuplicateLetters(s)
        print(result)


if __name__ == "__main__":
    main()
