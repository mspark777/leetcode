"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def summaryRanges(self, nums: list[int]) -> list[str]:
        result: list[str] = []

        i = 0
        while i < len(nums):
            head = nums[i]
            while True:
                j = i + 1
                if j >= len(nums):
                    break

                cur = nums[i] + 1
                next = nums[j]
                if cur != next:
                    break
                i = j

            tail = nums[i]
            if head == tail:
                result.append(str(head))
            else:
                result.append(str(head) + "->" + str(tail))

            i += 1

        return result


class Input:
    nums: list[int]

    def __init__(self, nums: list[int]):
        self.nums = nums


def main():
    inputs: list[Input] = [
        Input([0, 1, 2, 4, 5, 7]),
        Input([0, 2, 3, 4, 6, 8, 9]),
    ]

    solution = Solution()
    for input in inputs:
        nums = input.nums
        result = solution.summaryRanges(nums)
        print(result)


if __name__ == "__main__":
    main()
