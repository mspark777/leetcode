from typing import List


class Solution:
    def oddCells(self, m: int, n: int, indices: List[List[int]]) -> int:
        rows = [0] * m
        cols = [0] * n

        for index in indices:
            rows[index[0]] += 1
            cols[index[1]] += 1

        even_rows = m
        even_cols = n
        odd_rows = 0
        odd_cols = 0

        for cell in rows:
            if (cell & 1) == 1:
                odd_rows += 1
                even_rows -= 1

        for cell in cols:
            if (cell & 1) == 1:
                odd_cols += 1
                even_cols -= 1

        return odd_rows * even_cols + even_rows * odd_cols


class Input:
    m: int
    n: int
    indices: List[List[int]]

    def __init__(self, m: int, n: int, indices: list[list[int]]):
        self.m = m
        self.n = n
        self.indices = indices


def main():
    inputs = [Input(2, 3, [[0, 1], [1, 1]]), Input(2, 2, [[1, 1], [0, 0]])]

    for input in inputs:
        result = Solution().oddCells(input.m, input.n, input.indices)
        print(result)


if __name__ == "__main__":
    main()
