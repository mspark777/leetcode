from __future__ import annotations
from typing import Counter, List
from collections import Counter


class Solution:
    def gcd(self, a: int, b: int) -> int:
        return a if b == 0 else self.gcd(b, a % b)

    def maxPoints(self, points: List[List[int]]) -> int:
        N = len(points)
        if N < 2:
            return 1

        result = 2

        for i in range(N):
            slopes = Counter[str]()
            for j in range(N):
                if i == j:
                    continue

                [ix, iy] = points[i]
                [jx, jy] = points[j]
                x = jx - ix
                y = jy - iy
                gcd = self.gcd(abs(x), abs(y))
                if gcd != 0:
                    x //= gcd
                    y //= gcd

                key = "{}{}".format(x, y)
                slopes[key] += 1

            for count in slopes.values():
                result = max(result, count + 1)

        return result


def main():
    inputs: list[list[list[int]]] = [
        [[1, 1], [2, 2], [3, 3]],
        [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]],
    ]

    solution = Solution()
    for points in inputs:
        result = solution.maxPoints(points)
        print(result)


if __name__ == "__main__":
    main()
