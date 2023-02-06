from __future__ import annotations
from typing import List


class Solution:
    def shuffle(self, nums: List[int], n: int) -> List[int]:
        result = [0] * len(nums)

        for i in range(n):
            j = i * 2
            result[j] = nums[i]
            result[j + 1] = nums[n + i]

        return result


def main():
    inputs: list[tuple[list[int], int]] = [
        ([2, 5, 1, 3, 4, 7], 3),
        ([1, 2, 3, 4, 4, 3, 2, 1], 4),
    ]

    for nums, n in inputs:
        solution = Solution()
        result = solution.shuffle(nums, n)
        print(result)


if __name__ == "__main__":
    main()
