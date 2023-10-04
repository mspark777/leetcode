from __future__ import annotations
from typing import Optional, List


class MyHashMap:
    nums: list[int]

    def __init__(self):
        self.nums = [-1] * 1000001

    def put(self, key: int, value: int) -> None:
        self.nums[key] = value

    def get(self, key: int) -> int:
        return self.nums[key]

    def remove(self, key: int) -> None:
        self.nums[key] = -1


def main():
    obj = MyHashMap()
    obj.put(1, 1)
    obj.put(2, 2)
    print(obj.get(1))
    print(obj.get(3))
    obj.put(2, 1)
    print(obj.get(2))
    obj.remove(2)
    print(obj.get(2))


if __name__ == "__main__":
    main()
