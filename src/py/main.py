from __future__ import annotations
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
    input = (
        [
            [1, 3],
            [2, 3],
            [3, 6],
            [5, 6],
            [5, 7],
            [4, 5],
            [4, 8],
            [4, 9],
            [10, 4],
            [10, 9],
        ],
        [[2, 3], [1, 3], [5, 4], [6, 4]],
    )

    for matches in input:
        result = Solution().findWinners(matches)
        print(result)


if __name__ == "__main__":
    main()
