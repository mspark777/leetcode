from __future__ import annotations
from typing import Counter, List
from collections import Counter


class Solution:
    def minimumRounds(self, tasks: List[int]) -> int:
        counts = Counter(tasks)

        result = 0

        for count in counts.values():
            if count == 1:
                return -1

            result += count // 3
            if (count % 3) != 0:
                result += 1

        return result


def main():
    inputs: list[list[int]] = [[2, 2, 3, 3, 2, 4, 4, 4, 4, 4], [2, 3, 3]]

    solution = Solution()
    for tasks in inputs:
        result = solution.minimumRounds(tasks)
        print(result)


if __name__ == "__main__":
    main()
