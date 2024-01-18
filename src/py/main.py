from __future__ import annotations


class Solution:
    def climbStairs(self, n: int) -> int:
        n0 = 1
        n1 = 1

        for _ in range(1, n):
            sum = n0 + n1
            n0 = n1
            n1 = sum

        return n1


def main():
    input = (2, 3)

    for n in input:
        result = Solution().climbStairs(n)
        print(result)


if __name__ == "__main__":
    main()
