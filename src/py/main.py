from __future__ import annotations


class Solution:
    def knightDialer(self, n: int) -> int:
        if n == 1:
            return 10

        A = [
            [0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            [1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
        ]

        v = [[1] * 10]
        MOD = 10**9 + 7

        n -= 1
        while n:
            if n & 1:
                v = self.multiply(v, A)

            A = self.multiply(A, A)
            n >>= 1

        return sum(v[0]) % MOD

    def multiply(self, a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
        MOD = 10**9 + 7
        result = [[0] * len(b[0]) for _ in range(len(a))]
        for i in range(len(a)):
            for j in range(len(b[0])):
                for k in range(len(b)):
                    result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % MOD

        return result


def main():
    inputs = (1, 2, 3131)

    for n in inputs:
        result = Solution().knightDialer(n)
        print(result)


if __name__ == "__main__":
    main()
