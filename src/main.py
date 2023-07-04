from typing import List


class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        for i in range(len(nums)):
            while (
                (nums[i] > 0)
                and (nums[i] <= len(nums))
                and (nums[nums[i] - 1] != nums[i])
            ):
                temp = nums[i]
                nums[i] = nums[temp - 1]
                nums[temp - 1] = temp

        for i, num in enumerate(nums):
            if num != (i + 1):
                return i + 1

        return len(nums) + 1


def main():
    inputs = [[1, 2, 0], [3, 4, -1, 1], [7, 8, 9, 11, 12]]

    for nums in inputs:
        solution = Solution()
        result = solution.firstMissingPositive(nums)
        print(result)


if __name__ == "__main__":
    main()
