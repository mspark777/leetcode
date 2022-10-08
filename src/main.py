from __future__ import annotations
from typing import Optional, List


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()

        result = 0
        diffresult = 2147483647

        for i, ni in enumerate(nums):
            j = i + 1
            k = len(nums) - 1
            while j < k:
                nj = nums[j]
                nk = nums[k]
                sum = ni + nj + nk
                diffsum = abs(target - sum)

                if diffsum < diffresult:
                    result = sum
                    diffresult = diffsum

                if sum < target:
                    j += 1
                elif sum > target:
                    k -= 1
                else:
                    return sum

        return result


def main():
    inputs: list[tuple[list[int], int]] = [([-1, 2, 1, -4], 1), ([0, 0, 0], 1)]

    solution = Solution()
    for nums, target in inputs:
        result = solution.threeSumClosest(nums, target)
        print(result)


if __name__ == "__main__":
    main()
