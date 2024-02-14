from __future__ import annotations
from typing import List


class Solution:
    def rearrangeArray(self, nums: List[int]) -> List[int]:
        positive_index = 0
        negative_index = 1

        result = [0] * len(nums)
        for num in nums:
            if num > 0:
                result[positive_index] = num
                positive_index += 2
            else:
                result[negative_index] = num
                negative_index += 2

        return result


def main():
    input = ([3, 1, -2, -5, 2, -4], [-1, 1])

    for nums in input:
        result = Solution().rearrangeArray(nums)
        print(result)


if __name__ == "__main__":
    main()
