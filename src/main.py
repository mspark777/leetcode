"""
main
"""

from __future__ import annotations
from typing import Optional


class Solution:
    def findOriginalArray(self, changed: list[int]) -> list[int]:
        if (len(changed) % 2) == 1:
            return []

        queue: list[int] = []
        result: list[int] = []
        changed.sort()

        head = 0
        for i in changed:
            if len(queue) > head:
                if queue[head] == i:
                    head += 1
                else:
                    result.append(i)
                    queue.append(i * 2)
            else:
                result.append(i)
                queue.append(i * 2)

        return result if len(queue) == head else []


class Input:
    changed: list[int]

    def __init__(self, changed: list[int]):
        self.changed = changed


def main():
    inputs: list[Input] = [
        Input([1, 3, 4, 2, 6, 8]),
        Input([6, 3, 0, 1]),
        Input([1]),
        Input([0, 0, 0, 0]),
    ]

    solution = Solution()
    for input in inputs:
        changed = input.changed
        result = solution.findOriginalArray(changed)
        print(result)


if __name__ == "__main__":
    main()
