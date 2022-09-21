"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def sumEvenAfterQueries(
        self, nums: list[int], queries: list[list[int]]
    ) -> list[int]:
        sum = 0
        for num in nums:
            if (num % 2) == 0:
                sum += num

        result = [0 for _ in range(len(queries))]
        for i, query in enumerate(queries):
            val = query[0]
            index = query[1]
            num = nums[index]
            if (num % 2) == 0:
                sum -= num

            num += val
            if (num % 2) == 0:
                sum += num

            nums[index] = num
            result[i] = sum
        return result


class Input:
    nums: list[int]
    queries: list[list[int]]

    def __init__(self, nums: list[int], queries: list[list[int]]):
        self.nums = nums
        self.queries = queries


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 4], [[1, 0], [-3, 1], [-4, 0], [2, 3]]),
        Input([1], [[4, 0]]),
    ]

    solution = Solution()
    for input in inputs:
        nums = input.nums
        queries = input.queries
        result = solution.sumEvenAfterQueries(nums, queries)
        print(result)


if __name__ == "__main__":
    main()
