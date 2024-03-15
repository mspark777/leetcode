from __future__ import annotations
from typing import List


class Solution:
    def distributeCandies(self, candyType: List[int]) -> int:
        return min(len(set(candyType)), len(candyType) // 2)


def main():
    input = [[1, 1, 2, 2, 3, 3], [1, 1, 2, 3], [6, 6, 6, 6]]

    for candyType in input:
        result = Solution().distributeCandies(candyType)
        print(result)


if __name__ == "__main__":
    main()
