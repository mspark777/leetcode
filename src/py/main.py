from __future__ import annotations
from typing import List
from collections import Counter


class Solution:
    def minOperations(self, nums: List[int]) -> int:
        counts = Counter(nums)

        result = 0
        for count in counts.values():
            if count == 1:
                return -1

            result += int((count + 2) / 3)

        return result


def main():
    input = (
        [2, 3, 3, 2, 2, 4, 2, 3, 4],
        [2, 1, 2, 2, 3, 3],
        [14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12],
    )

    for nums in input:
        result = Solution().minOperations(nums)
        print(result)


if __name__ == "__main__":
    main()
