from __future__ import annotations
from typing import List


class Solution:
    def getSumAbsoluteDifferences(self, nums: List[int]) -> List[int]:
        n = len(nums)
        total_sum = sum(nums)
        left_sum = 0
        result: list[int] = []

        for i, num in enumerate(nums):
            right_sum = total_sum - left_sum - num

            left_count = i
            right_count = n - i - 1

            left_total = left_count * num - left_sum
            right_total = right_sum - right_count * num

            result.append(left_total + right_total)
            left_sum += num

        return result


def main():
    inputs = ([2, 3, 5], [1, 4, 6, 8, 10])

    for nums in inputs:
        result = Solution().getSumAbsoluteDifferences(nums)
        print(result)


if __name__ == "__main__":
    main()
