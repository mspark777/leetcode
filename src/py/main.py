from typing import List


class Solution:
    def findDifference(self, nums1: List[int], nums2: List[int]) -> List[List[int]]:
        return [self.filter(nums1, nums2), self.filter(nums2, nums1)]

    def filter(self, nums1: list[int], nums2: list[int]) -> list[int]:
        set1 = set(nums2)
        set2 = set([num for num in nums1 if not num in set1])
        return list(set2)


def main():
    inputs = [([1, 2, 3], [2, 4, 6]), ([1, 2, 3, 3], [1, 1, 2, 2])]

    for nums1, nums2 in inputs:
        solution = Solution()
        result = solution.findDifference(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
