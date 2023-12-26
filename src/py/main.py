from __future__ import annotations
from typing import List


class Solution:
    def thirdMax(self, nums: List[int]) -> int:
        min_value = -(2**32)
        first = min_value
        second = min_value
        third = min_value

        for num in nums:
            if num == first or num == second or num == third:
                continue

            if num > first:
                third = second
                second = first
                first = num
            elif num > second:
                third = second
                second = num
            elif num > third:
                third = num

        return third if third != min_value else first


def main():
    input = ([3, 2, 1], [1, 2], [2, 2, 3, 1])

    for nums in input:
        result = Solution().thirdMax(nums)
        print(result)


if __name__ == "__main__":
    main()
