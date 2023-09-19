from __future__ import annotations


class Solution:
    def findDuplicate(self, nums: list[int]) -> int:
        slow = nums[0]
        fast = nums[0]

        while True:
            slow = nums[slow]
            fast = nums[nums[fast]]
            if slow == fast:
                break

        slow = nums[0]
        while slow != fast:
            slow = nums[slow]
            fast = nums[fast]

        return slow


def main():
    inputs = [[1, 3, 4, 2, 2], [3, 1, 3, 4, 2]]

    for nums in inputs:
        solution = Solution()
        result = solution.findDuplicate(nums)
        print(result)


if __name__ == "__main__":
    main()
