from typing import List


class Solution:
    def twoOutOfThree(
        self, nums1: List[int], nums2: List[int], nums3: List[int]
    ) -> List[int]:
        return list(
            (set(nums1) & set(nums2))
            | (set(nums2) & set(nums3))
            | (set(nums1) & set(nums3))
        )


class Input:
    nums1: list[int]
    nums2: list[int]
    nums3: list[int]

    def __init__(self, nums1: list[int], nums2: list[int], nums3: list[int]):
        self.nums1 = nums1
        self.nums2 = nums2
        self.nums3 = nums3


def main():
    inputs = [
        Input([1, 1, 3, 2], [2, 3], [3]),
        Input([3, 1], [2, 3], [1, 2]),
        Input([1, 2, 2], [4, 3, 3], [5]),
    ]

    for input in inputs:
        result = Solution().twoOutOfThree(input.nums1, input.nums2, input.nums3)
        print(result)


if __name__ == "__main__":
    main()
