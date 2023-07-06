from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        left = 0
        sum = 0
        result = 2**31

        for right, num in enumerate(nums):
            sum += num
            while sum >= target:
                result = min(result, right - left + 1)
                sum -= nums[left]
                left += 1

        return result if result < (2**31) else 0


def main():
    inputs = [(7, [2, 3, 1, 2, 4, 3]), (4, [1, 4, 4]), (11, [1, 1, 1, 1, 1, 1, 1, 1])]

    for target, nums in inputs:
        solution = Solution()
        result = solution.minSubArrayLen(target, nums)
        print(result)


if __name__ == "__main__":
    main()
