from typing import List


class Solution:
    def ways(self, pizza: List[str], k: int) -> int:
        rows = len(pizza)
        cols = len(pizza[0])
        apples = self.matrix(rows + 1, cols + 1)
        f = self.matrix(rows, cols)

        for row in range(rows - 1, -1, -1):
            for col in range(cols - 1, -1, -1):
                apples[row][col] = (
                    self.btoi(pizza[row][col] == "A")
                    + apples[row + 1][col]
                    + apples[row][col + 1]
                    - apples[row + 1][col + 1]
                )
                f[row][col] = self.btoi(apples[row][col] > 0)

        mod = 1000000007
        for remain in range(1, k):
            g = self.matrix(rows, cols)
            for row in range(rows):
                for col in range(cols):
                    for next_row in range(row + 1, rows):
                        if (apples[row][col] - apples[next_row][col]) > 0:
                            g[row][col] += f[next_row][col]
                            g[row][col] %= mod

                    for next_col in range(col + 1, cols):
                        if (apples[row][col] - apples[row][next_col]) > 0:
                            g[row][col] += f[row][next_col]
                            g[row][col] %= mod
            f = g
        return f[0][0]

    def btoi(self, b: bool) -> int:
        return 1 if b else 0

    def matrix(self, rows: int, cols: int) -> list[list[int]]:
        return [[0 for j in range(cols)] for i in range(rows)]


def main():
    inputs: list[tuple[list[str], int]] = [
        (["A..", "AAA", "..."], 3),
        (["A..", "AA.", "..."], 3),
        (["A..", "A..", "..."], 1),
    ]

    for pizza, k in inputs:
        solution = Solution()
        result = solution.ways(pizza, k)
        print(result)


if __name__ == "__main__":
    main()
