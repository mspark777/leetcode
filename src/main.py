from __future__ import annotations
from typing import List


class Solution:
    def findLongestChain(self, pairs: List[List[int]]) -> int:
        pairs.sort(key=lambda x: x[1])

        result = 0
        pright = -1001
        for [left, right] in pairs:
            if pright < left:
                result += 1
                pright = right

        return result


def main():
    inputs = [[[1, 2], [2, 3], [3, 4]], [[1, 2], [7, 8], [4, 5]]]

    for pairs in inputs:
        solution = Solution()
        result = solution.findLongestChain(pairs)
        print(result)


if __name__ == "__main__":
    main()
