from __future__ import annotations


class Solution:
    def tribonacci(self, n: int) -> int:
        if n < 1:
            return 0
        elif n < 3:
            return 1

        return self.recursive(n, 0, 1, 1)

    def recursive(self, n: int, t0: int, t1: int, t2: int) -> int:
        return self.recursive(n - 1, t1, t2, t0 + t1 + t2) if n > 2 else t2


def main():
    inputs: list[int] = [4, 25]
    for n in inputs:
        solution = Solution()
        result = solution.tribonacci(n)
        print(result)


if __name__ == "__main__":
    main()
