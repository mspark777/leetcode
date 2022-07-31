"""
solution
"""
from __future__ import annotations
from typing import  Optional

def build_tree(nums: list[int], length: int) -> list[int]:
    tree = [0 for i in range(length * 2)]
    for i in range(length):
        tree[i + length] = nums[i]

    for i in range(length - 1, 0, -1):
        tree[i] = tree[i * 2] + tree[i * 2 + 1]

    return tree

class NumArray:
    length: int
    tree: list[int]
    def __init__(self, nums: list[int]):
        self.length = len(nums)
        self.tree = build_tree(nums, self.length)

    def update(self, index: int, val: int) -> None:
        index += self.length
        tree = self.tree

        tree[index] = val
        while index > 0:
            left = index
            right = index
            if index % 2 == 0:
                right = index + 1
            else:
                left = index - 1

            index = index // 2
            tree[index] = tree[left] + tree[right]


    def sumRange(self, left: int, right: int) -> int:
        left += self.length
        right += self.length
        tree = self.tree
        sum = 0
        while left <= right:
            if left % 2 == 1:
                sum += tree[left]
                left += 1

            if right % 2 == 0:
                sum += tree[right]
                right -= 1

            left //= 2
            right //= 2

        return sum
