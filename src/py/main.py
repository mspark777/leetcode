from __future__ import annotations
from typing import List


class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        nums_len = len(nums)
        checksums = [0] * nums_len
        for num in nums:
            checksums[num - 1] += 1

        missing = -1
        dup = -1

        for i, check in enumerate(checksums):
            if check == 0:
                missing = i + 1
            elif check > 1:
                dup = i + 1

            if missing > 0 and dup > 0:
                break

        return [dup, missing]


def main():
    input = ([1, 2, 2, 4], [1, 1])

    for nums in input:
        result = Solution().findErrorNums(nums)
        print(result)


if __name__ == "__main__":
    main()
