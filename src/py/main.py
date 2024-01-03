from __future__ import annotations
from typing import List


class Solution:
    def nextGreaterElement(self, nums1: List[int], nums2: List[int]) -> List[int]:
        stack: list[int] = []
        nums3 = dict[int, int]()

        for i in range(len(nums2)):
            while stack and nums2[stack[-1]] < nums2[i]:
                nums3[nums2[stack[-1]]] = nums2[i]
                stack.pop()
            stack.append(i)

        result: list[int] = []

        for num in nums1:
            if num in nums3:
                result.append(nums3[num])
            else:
                result.append(-1)

        return result


def main():
    input = (([4, 1, 2], [1, 3, 4, 2]), ([2, 4], [1, 2, 3, 4]))

    for (
        nums1,
        nums2,
    ) in input:
        result = Solution().nextGreaterElement(nums1, nums2)
        print(result)


if __name__ == "__main__":
    main()
