from __future__ import annotations
from typing import List


class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        pairs = [(scores[i], ages[i]) for i in range(len(ages))]
        pairs.sort()

        higeat_age = 0
        for age in ages:
            higeat_age = max(higeat_age, age)

        bits = [0] * (higeat_age + 1)
        result = -(2 ** 31)

        for score, age in pairs:
            best = score + self.query(bits, age)
            self.update(bits, age, best)

            result = max(result, best)

        return result

    def update(self, bits: list[int], age: int, best: int):
        i = age
        while i < len(bits):
            bits[i] = max(bits[i], best)
            i += i & (-i)

    def query(self, bits: list[int], age: int) -> int:
        max_score = -(2 ** 31)
        i = age
        while i > 0:
            max_score = max(max_score, bits[i])
            i -= i & (-i)

        return max_score


def main():
    inputs: list[list[list[int]]] = [
    [[1, 3, 5, 10, 15], [1, 2, 3, 4, 5]],
    [[4, 5, 6, 5], [2, 1, 2, 1]],
    [[1, 2, 3, 5], [8, 9, 10, 1]]

    ]
    for [scores, ages] in inputs:
        solution = Solution()
        result = solution.bestTeamScore(scores, ages)
        print(result)


if __name__ == "__main__":
    main()
