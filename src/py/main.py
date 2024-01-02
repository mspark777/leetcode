from __future__ import annotations
from typing import List


class Solution:
    def findMatrix(self, nums: List[int]) -> List[List[int]]:
        num_map = dict[int, list[int]]()
        for num in nums:
            if num in num_map:
                num_map[num].append(num)
            else:
                num_map[num] = [num]

        result: list[list[int]] = []
        while num_map:
            row: list[int] = []
            empty_nums: list[int] = []
            for nums in num_map.values():
                num = nums.pop()
                row.append(num)
                if len(nums) < 1:
                    empty_nums.append(num)

            for num in empty_nums:
                num_map.pop(num)

            result.append(row)

        return result


def main():
    input = ([1, 3, 4, 1, 2, 3, 1], [1, 2, 3, 4])

    for nums in input:
        result = Solution().findMatrix(nums)
        print(result)


if __name__ == "__main__":
    main()
