from typing import List


class Solution:
    def minCost(self, nums: List[int], cost: List[int]) -> int:
        left = 1000001
        right = 0
        for num in nums:
            left = min(left, num)
            right = max(right, num)

        result = self.get_cost(nums, cost, nums[0])
        while left < right:
            mid = (left + right) // 2
            cost1 = self.get_cost(nums, cost, mid)
            cost2 = self.get_cost(nums, cost, mid + 1)
            result = min(cost1, cost2)

            if cost1 > cost2:
                left = mid + 1
            else:
                right = mid

        return result

    def get_cost(self, nums: list[int], cost: list[int], base: int) -> int:
        result = 0
        for i, num in enumerate(nums):
            result += abs(num - base) * cost[i]

        return result


def main():
    inputs = [([1, 3, 5, 2], [2, 3, 1, 14]), ([2, 2, 2, 2, 2], [4, 2, 8, 1, 3])]

    for nums, cost in inputs:
        solution = Solution()
        result = solution.minCost(nums, cost)
        print(result)


if __name__ == "__main__":
    main()
