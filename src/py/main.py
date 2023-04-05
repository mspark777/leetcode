from typing import List


class Solution:
    def minimizeArrayValue(self, nums: List[int]) -> int:
        result = 0
        prefix_sum = 0

        for i, num in enumerate(nums):
            prefix_sum += num
            result = max(result, (prefix_sum + i) // (i + 1))

        return result


def main():
    inputs: list[list[int]] = [[3, 7, 1, 6], [10, 1]]

    for nums in inputs:
        solution = Solution()
        result = solution.minimizeArrayValue(nums)
        print(result)


if __name__ == "__main__":
    main()
