from __future__ import annotations
from typing import List


class Solution:
    def sortJumbled(self, mapping: List[int], nums: List[int]) -> List[int]:
        store_pairs: list[tuple[int, int]] = []

        for i, num in enumerate(nums):
            if num == 0:
                store_pairs.append((mapping[0], i))
                continue

            mapped_value = 0
            place = 1

            while num != 0:
                mapped_value = place * mapping[num % 10] + mapped_value
                place *= 10
                num //= 10

            store_pairs.append((mapped_value, i))

        store_pairs.sort()

        return [nums[pair[1]] for pair in store_pairs]


def main():
    inputs: list[tuple[list[int], list[int]]] = [
        ([8, 9, 4, 0, 2, 1, 3, 5, 7, 6], [991, 338, 38]),
        ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [789, 456, 123]),
    ]

    for mapping, nums in inputs:
        result = Solution().sortJumbled(mapping, nums)
        print(result)


if __name__ == "__main__":
    main()
