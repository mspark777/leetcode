from typing import List


class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        zero_count = 0
        result = 0
        start = 0

        for i, num in enumerate(nums):
            zero_count += 1 if num == 0 else 0

            while zero_count > 1:
                zero_count -= 1 if nums[start] == 0 else 0
                start += 1

            result = max(result, i - start)

        return result


def main():
    inputs = [[1, 1, 0, 1], [0, 1, 1, 1, 0, 1, 1, 0, 1], [1, 1, 1]]

    for nums in inputs:
        solution = Solution()
        result = solution.longestSubarray(nums)
        print(result)


if __name__ == "__main__":
    main()
