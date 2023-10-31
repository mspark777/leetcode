from __future__ import annotations
from typing import Optional, List


class Solution:
    def findArray(self, pref: List[int]) -> List[int]:
        n = len(pref)

        for i in range(n - 1, 0, -1):
            pref[i] = pref[i] ^ pref[i - 1]

        return pref


def main():
    inputs = ([5, 2, 0, 3, 1], [13])

    for pref in inputs:
        result = Solution().findArray(pref)
        print(result)


if __name__ == "__main__":
    main()
