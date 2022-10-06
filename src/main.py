from __future__ import annotations
from typing import Optional, List


class TimeMap:
    store: dict[str, list[tuple[str, int]]]

    def __init__(self):
        self.store = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key not in self.store:
            self.store[key] = []

        self.store[key].append((value, timestamp))

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.store:
            return ""

        nodes = self.store[key]
        if timestamp < nodes[0][1]:
            return ""

        left = 0
        right = len(nodes)

        while left < right:
            mid = (left + right) // 2
            if nodes[mid][1] <= timestamp:
                left = mid + 1
            else:
                right = mid

        if right == 0:
            return ""

        return nodes[right - 1][0]


def main():
    timemap = TimeMap()
    timemap.set("foo", "bar", 1)
    print(timemap.get("foo", 1))
    print(timemap.get("foo", 3))
    timemap.set("foo", "bar2", 4)
    print(timemap.get("foo", 4))
    print(timemap.get("foo", 5))


if __name__ == "__main__":
    main()
