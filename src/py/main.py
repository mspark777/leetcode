from __future__ import annotations
from typing import List
from collections import defaultdict


class Solution:
    def garbageCollection(self, garbage: List[str], travel: List[int]) -> int:
        for i in range(1, len(travel)):
            travel[i] = travel[i - 1] + travel[i]

        last_pos = defaultdict[str, int](int)
        result = 0

        for i, g in enumerate(garbage):
            for c in g:
                last_pos[c] = i
            result += len(g)

        types = "MPG"
        for t in types:
            if last_pos[t] != 0:
                result += travel[last_pos[t] - 1]

        return result


def main():
    inputs = ((["G", "P", "GP", "GG"], [2, 4, 3]), (["MMM", "PGM", "GP"], [3, 10]))

    for garbage, travel in inputs:
        result = Solution().garbageCollection(garbage, travel)
        print(result)


if __name__ == "__main__":
    main()
