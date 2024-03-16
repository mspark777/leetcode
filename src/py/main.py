from __future__ import annotations
from typing import List


class Solution:
    def findMaxLength(self, nums: List[int]) -> int:
        count = 0
        maxlen = 0
        map = {0: -1}
        for i, num in enumerate(nums):
            count = count + (1 if num == 1 else -1)
            if count in map:
                maxlen = max(maxlen, i - map[count])
            else:
                map[count] = i
        return maxlen


def main():
    input = [[0, 1], [0, 1, 0]]

    for nums in input:
        result = Solution().findMaxLength(nums)
        print(result)


if __name__ == "__main__":
    main()
