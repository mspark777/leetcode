from typing import List


class Solution:
    def maxUncrossedLines(self, nums1: List[int], nums2: List[int]) -> int:
        len1 = len(nums1)
        len2 = len(nums2)
        dp = [0 for i in range(len2 + 1)]
        dp_prev = [0 for i in range(len2 + 1)]

        for i in range(1, len1 + 1):
            for j in range(1, len2 + 1):
                if nums1[i - 1] == nums2[j - 1]:
                    dp[j] = 1 + dp_prev[j - 1]
                else:
                    dp[j] = max(dp[j - 1], dp_prev[j])

            dp_prev = dp.copy()

        return dp[len2]


def main():
    inputs = [
        ([1, 4, 2], [1, 2, 4]),
        ([2, 5, 1, 2, 5], [10, 5, 2, 1, 5, 2]),
        ([1, 3, 7, 1, 7, 5], [1, 9, 2, 5, 1]),
        ([3, 2], [2, 2, 2, 3]),
    ]

    for nums1, nums2 in inputs:
        solution = Solution()
        result = solution.maxUncrossedLines(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
