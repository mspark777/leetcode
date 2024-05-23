from __future__ import annotations
from collections import defaultdict
from typing import List


class Solution:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        result = 1

        frequencies_map = defaultdict[int, dict[int, int]](dict)
        for num in nums:
            key = num % k
            frequencies_map[key][num] = frequencies_map[key].get(num, 0) + 1

        for frequencies in frequencies_map.values():
            prev_num = -k
            curr = 1
            prev1 = 1
            prev2 = 0

            for num, frequency in sorted(frequencies.items()):
                skip = prev1

                if num - prev_num == k:
                    take = ((1 << frequency) - 1) * prev2
                else:
                    take = ((1 << frequency) - 1) * prev1

                curr = skip + take
                prev2 = prev1
                prev1 = curr
                prev_num = num

            result *= curr

        return result - 1


def main():
    input: list[tuple[list[int], int]] = [
        ([2, 4, 6], 2),
        ([1], 1),
    ]

    for nums, k in input:
        result = Solution().beautifulSubsets(nums, k)
        print(result)


if __name__ == "__main__":
    main()
