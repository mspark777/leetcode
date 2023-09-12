from __future__ import annotations


class Solution:
    def subsetsWithDup(self, nums: list[int]) -> list[list[int]]:
        result: list[list[int]] = []
        nums.sort()
        self.solve(0, nums, [], result)

        return result

    def solve(self, i: int, nums: list[int], temp: list[int], result: list[list[int]]):
        result.append(temp.copy())

        for j in range(i, len(nums)):
            if (j > i) and (nums[j] == nums[j - 1]):
                continue

            temp.append(nums[j])
            self.solve(j + 1, nums, temp, result)
            temp.pop()


def main():
    inputs = [[1, 2, 2], [0], [4, 4, 4, 1, 4]]

    for nums in inputs:
        solution = Solution()
        result = solution.subsetsWithDup(nums)
        print(result)


if __name__ == "__main__":
    main()
