from __future__ import annotations
from typing import List


class Solution:
    def rangeSum(self, nums: List[int], n: int, left: int, right: int) -> int:
        mod = 10**9 + 7
        sum_left = self.sum_of_first_k(nums, n, left - 1)
        sum_right = self.sum_of_first_k(nums, n, right)
        result = (sum_right - sum_left) % mod
        return (result + mod) % mod

    def count_and_sum(self, nums: list[int], n: int, target: int):
        count = 0
        current_sum = 0
        total_sum = 0
        window_sum = 0
        i = 0
        for j in range(n):
            current_sum += nums[j]
            window_sum += nums[j] * (j - i + 1)
            while current_sum > target:
                window_sum -= current_sum
                current_sum -= nums[i]
                i += 1
            count += j - i + 1
            total_sum += window_sum
        return count, total_sum

    def sum_of_first_k(self, nums: list[int], n: int, k: int):
        min_sum = min(nums)
        max_sum = sum(nums)
        left = min_sum
        right = max_sum

        while left <= right:
            mid = left + (right - left) // 2
            if self.count_and_sum(nums, n, mid)[0] >= k:
                right = mid - 1
            else:
                left = mid + 1
        count, total_sum = self.count_and_sum(nums, n, left)
        return total_sum - left * (count - k)


class Input:
    nums: list[int]
    n: int
    left: int
    right: int

    def __init__(self, nums: list[int], n: int, left: int, right: int):
        self.nums = nums
        self.n = n
        self.left = left
        self.right = right


def main():
    inputs: list[Input] = [
        Input([1, 2, 3, 4], 4, 1, 5),
        Input([1, 2, 3, 4], 4, 3, 4),
        Input([1, 2, 3, 4], 4, 1, 10),
    ]

    for input in inputs:
        result = Solution().rangeSum(input.nums, input.n, input.left, input.right)
        print(result)


if __name__ == "__main__":
    main()
