from __future__ import annotations
from typing import List


class Solution:
    def maximumBags(
        self, capacity: List[int], rocks: List[int], additionalRocks: int
    ) -> int:
        remains = list(map(lambda a, b: a - b, capacity, rocks))
        remains.sort()

        result = 0
        for remain in remains:
            if additionalRocks >= remain:
                additionalRocks -= remain
                result += 1
            else:
                break

        return result


class Input:
    capacity: list[int]
    rocks: list[int]
    additional_rocks: int

    def __init__(
        self, capacity: list[int], rocks: list[int], additional_rocks: int
    ) -> None:
        self.capacity = capacity
        self.rocks = rocks
        self.additional_rocks = additional_rocks


def main():
    inputs: list[Input] = [
        Input([2, 3, 4, 5], [1, 2, 4, 4], 2),
        Input([10, 2, 2], [2, 2, 0], 100),
    ]

    solution = Solution()
    for input in inputs:
        result = solution.maximumBags(
            input.capacity, input.rocks, input.additional_rocks
        )
        print(result)


if __name__ == "__main__":
    main()
