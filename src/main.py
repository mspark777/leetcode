from __future__ import annotations
from typing import List


class NumArray:
    sums: list[int]

    def __init__(self, nums: List[int]):
        self.sums = [0]
        for num in nums:
            self.sums.append(self.sums[-1] + num)

    def sumRange(self, left: int, right: int) -> int:
        return self.sums[right + 1] - self.sums[left]


def main():
    obj = NumArray([-2, 0, 3, -5, 2, -1])
    print(obj.sumRange(0, 2))
    print(obj.sumRange(2, 5))
    print(obj.sumRange(0, 5))


if __name__ == "__main__":
    main()
