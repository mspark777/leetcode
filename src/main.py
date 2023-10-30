from __future__ import annotations
from typing import Optional, List


class Solution:
    def sortByBits(self, arr: List[int]) -> List[int]:
        arr.sort(key=lambda n: (n.bit_count(), n))
        return arr


def main():
    inputs = (
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
    )

    for arr in inputs:
        result = Solution().sortByBits(arr)
        print(result)


if __name__ == "__main__":
    main()
