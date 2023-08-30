from __future__ import annotations
from typing import List


class Solution:
    def minimumReplacement(self, nums: List[int]) -> int:
        result = 0

        for i in range(len(nums) - 2, -1, -1):
            n0 = nums[i]
            n1 = nums[i + 1]
            if n0 <= n1:
                continue

            num_elements = (n0 + n1 - 1) // n1
            result += num_elements - 1
            nums[i] = n0 // num_elements

        return result


def main():
    inputs = [[3, 9, 3], [1, 2, 3, 4, 5]]

    for nums in inputs:
        solution = Solution()
        result = solution.minimumReplacement(nums)
        print(result)


if __name__ == "__main__":
    main()
