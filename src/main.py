from __future__ import annotations
from typing import List


class Solution:
    def groupThePeople(self, group_sizes: List[int]) -> List[List[int]]:
        result: list[list[int]] = []

        groups: dict[int, list[int]] = {}
        for i, size in enumerate(group_sizes):
            if size not in groups:
                groups[size] = []

            group = groups[size]
            group.append(i)

            if len(group) == size:
                result.append(group)
                groups.pop(size)

        return result


def main():
    inputs = [[3, 3, 3, 3, 3, 1, 3], [2, 1, 3, 3, 3, 2]]

    for group_sizes in inputs:
        solution = Solution()
        result = solution.groupThePeople(group_sizes)
        print(result)


if __name__ == "__main__":
    main()
