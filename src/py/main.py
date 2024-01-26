from __future__ import annotations


class Solution:
    def findPaths(
        self, m: int, n: int, max_move: int, start_row: int, start_column: int
    ) -> int:
        MODULO = 1000000007
        dp = [[0 for _ in range(n)] for _ in range(m)]
        dp[start_row][start_column] = 1
        result = 0

        for _ in range(max_move):
            temp = [[0 for _ in range(n)] for _ in range(m)]
            for r in range(m):
                for c in range(n):
                    if r == (m - 1):
                        result = (result + dp[r][c]) % MODULO

                    if c == (n - 1):
                        result = (result + dp[r][c]) % MODULO

                    if r == 0:
                        result = (result + dp[r][c]) % MODULO

                    if c == 0:
                        result = (result + dp[r][c]) % MODULO

                    tr = 0
                    if r > 0:
                        tr += dp[r - 1][c]

                    if r < (m - 1):
                        tr += dp[r + 1][c]
                    tr %= MODULO

                    tc = 0
                    if c > 0:
                        tc += dp[r][c - 1]

                    if c < (n - 1):
                        tc += dp[r][c + 1]
                    tc %= MODULO

                    temp[r][c] = (tr + tc) % MODULO
            dp = temp

        return result


def main():
    input = ((2, 2, 2, 0, 0), (1, 3, 3, 0, 1))

    for m, n, max_move, start_row, start_column in input:
        result = Solution().findPaths(m, n, max_move, start_row, start_column)
        print(result)


if __name__ == "__main__":
    main()
