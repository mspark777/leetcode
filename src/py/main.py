from __future__ import annotations
from typing import List


class Solution:
    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        n = len(nums)
        dis_list = [i + 1 for i in range(n)]

        for num in nums:
            dis_list[num - 1] = -1

        return [i for i in dis_list if i >= 0]


def main():
    input = ([4, 3, 2, 7, 8, 2, 3, 1], [1, 1])

    for nums in input:
        result = Solution().findDisappearedNumbers(nums)
        print(result)


if __name__ == "__main__":
    main()
