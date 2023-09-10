from __future__ import annotations


class Solution:
    def countOrders(self, n: int) -> int:
        result = 1
        MOD = (10**9) + 7
        for i in range(2, n + 1):
            result = (result * (2 * i - 1) * i) % MOD

        return result


def main():
    inputs = [1, 2, 3]

    for n in inputs:
        solution = Solution()
        result = solution.countOrders(n)
        print(result)


if __name__ == "__main__":
    main()
