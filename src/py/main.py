from __future__ import annotations
from typing import List


class Solution:
    def getCommon(self, nums1: List[int], nums2: List[int]) -> int:
        i1 = 0
        i2 = 0

        while i1 < len(nums1) and i2 < len(nums2):
            n1 = nums1[i1]
            n2 = nums2[i2]

            if n1 < n2:
                i1 += 1
            elif n1 > n2:
                i2 += 1
            else:
                return n1

        return -1


def main():
    input = [([1, 2, 3], [2, 4]), ([1, 2, 3, 6], [2, 3, 4, 5])]
    for nums1, nums2 in input:
        result = Solution().getCommon(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
