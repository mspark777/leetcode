from typing import List
from functools import reduce


class Solution:
    def getAverages(self, nums: List[int], k: int) -> List[int]:
        if k < 1:
            return nums

        result = [-1] * len(nums)
        window_len = (k * 2) + 1
        if window_len > len(nums):
            return result

        window_sum = reduce(lambda x, y: x + y, nums[0:window_len])
        result[k] = window_sum // window_len

        for i in range(window_len, len(nums)):
            window_sum = window_sum - nums[i - window_len] + nums[i]
            result[i - k] = window_sum // window_len

        return result


def main():
    inputs = [([7, 4, 3, 9, 1, 8, 5, 2, 6], 3), ([100000], 0), ([8], 100000)]

    for nums, k in inputs:
        solution = Solution()
        result = solution.getAverages(nums, k)
        print(result)


if __name__ == "__main__":
    main()
