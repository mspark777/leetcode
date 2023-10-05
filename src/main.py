from __future__ import annotations
from typing import Optional, List


class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        if not nums:
            return []

        candidate0 = 0
        candidate1 = 1
        count0 = 0
        count1 = 0

        for num in nums:
            if num == candidate0:
                count0 += 1
            elif num == candidate1:
                count1 += 1
            elif count0 == 0:
                candidate0 = num
                count0 = 1
            elif count1  == 0:
                candidate1 = num
                count1 = 1
            else:
                count0 -= 1
                count1 -= 1

        return [num for num in (candidate0, candidate1) if nums.count(num) > len(nums) // 3]

def main():
    inputs = ([3, 2, 3], [1], [1, 2])

    for nums in inputs:
        result = Solution().majorityElement(nums)
        print(result)


if __name__ == "__main__":
    main()
