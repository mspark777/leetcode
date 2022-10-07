from __future__ import annotations
from typing import Optional, List


class MyMap:
    m: dict[int, int]

    def __init__(self) -> None:
        self.m = {}

    def set(self, key: int, val: int):
        self.m[key] = val

    def get(self, key: int) -> int:
        return self.m[key] if key in self.m else 0


class MyCalendarThree:
    vals: MyMap
    lazy: MyMap

    def __init__(self):
        self.vals = MyMap()
        self.lazy = MyMap()

    def update(self, start: int, end: int, left: int, right: int, idx: int):
        if (start > right) or (end < left):
            return

        vals = self.vals
        lazy = self.lazy

        if (start <= left) and (right <= end):
            vals.set(idx, vals.get(idx) + 1)
            lazy.set(idx, lazy.get(idx) + 1)
        else:
            mid = (left + right) // 2
            idx2 = idx * 2
            idx21 = idx2 + 1

            self.update(start, end, left, mid, idx2)
            self.update(start, end, mid + 1, right, idx21)

            temp = max(vals.get(idx2), vals.get(idx21))
            vals.set(idx, lazy.get(idx) + temp)

    def book(self, start: int, end: int) -> int:
        self.update(start, end - 1, 0, 1000000000, 1)
        return self.vals.get(1)


def main():
    obj = MyCalendarThree()
    print(obj.book(10, 20))
    print(obj.book(50, 60))
    print(obj.book(10, 40))
    print(obj.book(5, 15))
    print(obj.book(5, 10))
    print(obj.book(25, 55))


if __name__ == "__main__":
    main()
