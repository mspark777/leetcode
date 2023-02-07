from __future__ import annotations
from typing import List


class Solution:
    def totalFruit(self, fruits: List[int]) -> int:
        baskets: dict[int, int] = {}
        left = 0
        result = 0

        for right, rfruit in enumerate(fruits):
            baskets[rfruit] = baskets.get(rfruit, 0) + 1

            while len(baskets) > 2:
                lfruit = fruits[left]
                baskets[lfruit] -= 1
                if baskets[lfruit] == 0:
                    del baskets[lfruit]
                left += 1

            result = max(result, right - left + 1)

        return result


def main():
    inputs: list[list[int]] = [
        [1, 2, 1],
        [0, 1, 2, 2],
        [1, 2, 3, 2, 2],
        [3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4],
    ]

    for fruits in inputs:
        solution = Solution()
        result = solution.totalFruit(fruits)
        print(result)


if __name__ == "__main__":
    main()
