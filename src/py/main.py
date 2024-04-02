from __future__ import annotations
from typing import List


class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        result: List[int] = []
        for num in nums:
            i = abs(num) - 1

            if nums[i] < 0:
                result.append(i + 1)
            else:
                nums[i] = -nums[i]

        return result


def main():
    input = [[4, 3, 2, 7, 8, 2, 3, 1], [1, 1, 2], [1]]

    for nums in input:
        result = Solution().findDuplicates(nums)
        print(result)


if __name__ == "__main__":
    main()
