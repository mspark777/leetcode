from __future__ import annotations
from typing import Optional, List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        if len(nums1) > len(nums2):
            return self.findMedianSortedArrays(nums2, nums1)

        m, n = len(nums1), len(nums2)
        left, right = 0, m

        while left <= right:
            parition_a = (left + right) // 2
            parition_b = ((m + n + 1) // 2) - parition_a

            max_left_a = float("-inf") if parition_a == 0 else nums1[parition_a - 1]
            max_left_b = float("-inf") if parition_b == 0 else nums2[parition_b - 1]
            min_right_a = float("inf") if parition_a == m else nums1[parition_a]
            min_right_b = float("inf") if parition_b == n else nums2[parition_b]

            if (max_left_a <= min_right_b) and (max_left_b <= min_right_a):
                if ((m + n) % 2) == 0:
                    return (
                        max(max_left_a, max_left_b) + min(min_right_a, min_right_b)
                    ) / 2
                else:
                    return max(max_left_a, max_left_b)
            elif max_left_a > min_right_b:
                right = parition_a - 1
            else:
                left = parition_a + 1

        return float("inf")


def main():
    inputs = [([1, 3], [2]), ([1, 2], [3, 4])]

    for nums1, nums2 in inputs:
        solution = Solution()
        result = solution.findMedianSortedArrays(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
