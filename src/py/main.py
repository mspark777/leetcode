from __future__ import annotations
from typing import List


class Solution:
    def matrixReshape(self, mat: List[List[int]], r: int, c: int) -> List[List[int]]:
        if len(mat) * len(mat[0]) != r * c:
            return mat

        result = [([0] * c) for _ in range(r)]

        dr = 0
        dc = 0
        for row in mat:
            for col in row:
                result[dr][dc] = col
                dc += 1
                if dc == c:
                    dc = 0
                    dr += 1

        return result


def main():
    input = (([[1, 2], [3, 4]], 1, 4), ([[1, 2], [3, 4]], 2, 4))

    for mat, r, c in input:
        result = Solution().matrixReshape(mat, r, c)
        print(result)


if __name__ == "__main__":
    main()
