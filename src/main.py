from __future__ import annotations
from typing import Callable, Deque, Optional, List
from collections import Counter, deque
from random import randrange


class RandomizedSet:
    nums: list[int]
    indexes: dict[int, int]

    def __init__(self):
        self.nums = []
        self.indexes = {}

    def insert(self, val: int) -> bool:
        if val in self.indexes:
            return False

        self.indexes[val] = len(self.nums)
        self.nums.append(val)
        return True

    def remove(self, val: int) -> bool:
        if val not in self.indexes:
            return False

        last = self.nums[-1]
        pos = self.indexes[val]

        self.indexes[last] = pos
        self.nums[pos] = last

        self.indexes.pop(val)
        self.nums.pop()

        return True

    def getRandom(self) -> int:
        index = randrange(0, len(self.nums))
        return self.nums[index]


def main():
    obj = RandomizedSet()
    print(obj.insert(3))
    print(obj.insert(3))
    print(obj.getRandom())
    print(obj.getRandom())
    print(obj.insert(1))
    print(obj.remove(3))
    print(obj.getRandom())
    print(obj.insert(0))
    print(obj.remove(0))


if __name__ == "__main__":
    main()
