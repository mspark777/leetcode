from __future__ import annotations
from typing import List


class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        g.sort()
        s.sort()

        child = 0
        cookie = 0

        while child < len(g) and cookie < len(s):
            if g[child] <= s[cookie]:
                child += 1

            cookie += 1

        return child


def main():
    input = (([1, 2, 3], [1, 1]), ([1, 2], [1, 2, 3]))

    for g, s in input:
        result = Solution().findContentChildren(g, s)
        print(result)


if __name__ == "__main__":
    main()
