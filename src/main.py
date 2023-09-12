from __future__ import annotations
from collections import Counter


class Solution:
    def minDeletions(self, s: str) -> int:
        counts = Counter(s)
        result = 0
        seen: set[int] = set()
        for count in counts.values():
            while count > 0 and count in seen:
                result += 1
                count -= 1
            seen.add(count)

        return result


def main():
    inputs = ["aab", "aaabbbcc", "ceabaacb"]

    for s in inputs:
        solution = Solution()
        result = solution.minDeletions(s)
        print(result)


if __name__ == "__main__":
    main()
