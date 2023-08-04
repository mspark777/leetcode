from __future__ import annotations


class Solution:
    def myPow(self, x: float, n: int) -> float:
        if n == 0:
            return 1

        if n < 0:
            n *= -1
            x = 1 / x

        result = 1.0

        while n != 0:
            if (n & 1) == 1:
                result *= x
                n -= 1

            x *= x
            n //= 2

        return result


def main():
    inputs = [(2.00000, 10), (2.10000, 3), (2.00000, -2)]

    for x, n in inputs:
        solution = Solution()
        result = solution.myPow(x, n)
        print(result)


if __name__ == "__main__":
    main()
