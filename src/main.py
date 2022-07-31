"""
main
"""

from typing import Optional
from solution import NumArray


class Input:
    nums: list[int]
    def __init__(self, nums: list[int]):
        self.nums = nums

def main():
    inputs: list[Input] = [
            Input(nums = [-1])
    ]

    for i in inputs:
        narr = NumArray(i.nums)
        print(narr.sumRange(0, 0))
        narr.update(0, 1)
        print(narr.sumRange(0, 0))



if __name__ == "__main__":
    main()
