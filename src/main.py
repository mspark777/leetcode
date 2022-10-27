from __future__ import annotations
from typing import Optional, List


class Solution:
    def largestOverlap(self, img1: List[List[int]], img2: List[List[int]]) -> int:
        N = len(img1)
        BN = (3 * N) - 2
        bpadded = [[0 for x in range(BN)] for x in range(BN)]

        for r in range(N):
            for c in range(N):
                bpadded[r + N - 1][c + N - 1] = img2[r][c]

        SN = 2 * N - 1
        max_overlaps = 0
        for xshift in range(SN):
            for yshift in range(SN):
                max_overlaps = max(
                    max_overlaps, self.convolute(img1, bpadded, xshift, yshift)
                )

        return max_overlaps

    def convolute(
        self, img: list[list[int]], kernel: list[list[int]], xshift: int, yshift: int
    ) -> int:
        N = len(img)

        result = 0
        for r in range(N):
            for c in range(N):
                result += img[r][c] * kernel[r + yshift][c + xshift]

        return result


def main():
    inputs: list[tuple[list[list[int]], list[list[int]]]] = [
        ([[1, 1, 0], [0, 1, 0], [0, 1, 0]], [[0, 0, 0], [0, 1, 1], [0, 0, 1]]),
        ([[1]], [[1]]),
        ([[0]], [[0]]),
        ([[0, 0, 0], [1, 1, 0], [0, 0, 0]], [[0, 1, 1], [0, 0, 0], [0, 0, 0]]),
    ]

    solution = Solution()
    for img1, img2 in inputs:
        result = solution.largestOverlap(img1, img2)
        print(result)


if __name__ == "__main__":
    main()
