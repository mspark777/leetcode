from __future__ import annotations
from typing import List


class Solution:
    def findJudge(self, n: int, trust: List[List[int]]) -> int:
        counts = [0] * n
        for [f, t] in trust:
            counts[f - 1] -= 1
            counts[t - 1] += 1

        judge = n - 1
        for person, count in enumerate(counts):
            if count == judge:
                return person + 1

        return -1


def main():
    inputs: list[tuple[int, list[list[int]]]] = [
        (2, [[1, 2]]),
        (3, [[1, 3], [2, 3]]),
        (3, [[1, 3], [2, 3], [3, 1]]),
        (1, []),
    ]
    for n, trust in inputs:
        solution = Solution()
        result = solution.findJudge(n, trust)
        print(result)


if __name__ == "__main__":
    main()
